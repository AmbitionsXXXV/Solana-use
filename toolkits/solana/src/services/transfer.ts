import {
	Connection,
	Keypair,
	PublicKey,
	SystemProgram,
	type TransactionInstruction,
	ComputeBudgetProgram,
	TransactionMessage,
	VersionedTransaction,
} from "@solana/web3.js"
import { type LogMessage, logger } from "@xxhh/toolkits-logger"
import fs from "node:fs"
import { COMMITMENT } from "../constant"
import type { TransferConfig, TransferResult } from "../types/transfer"

/**
 * // -- 使用示例
 * const transferManager = new TransferManager(walletPath, SOLANA_RPC_URL)
 * // 单笔转账
 * const result = await transferManager.transfer(
 *     "接收地址",
 *     1000, // lamports
 *     {
 *         computeUnitPrice: DEFAULT_COMPUTE_UNIT_PRICE,
 *         computeUnitLimit: DEFAULT_COMPUTE_UNIT_LIMIT,
 *     }
 * )
 *
 * // 批量转账
 * const results = await transferManager.batchTransfer(
 *     "接收地址",
 *     1000, // lamports
 *     5, // 转账次数
 *     {
 *         computeUnitPrice: DEFAULT_COMPUTE_UNIT_PRICE,
 *         computeUnitLimit: DEFAULT_COMPUTE_UNIT_LIMIT,
 *     }
 * )
 */
export class TransferManager {
	private connection: Connection
	public wallet: Keypair

	constructor(walletKeyPath: string, endpoint: string) {
		// -- 创建 RPC 连接
		this.connection = new Connection(endpoint, COMMITMENT)

		// -- 从文件加载钱包
		const secretKeyString = fs.readFileSync(walletKeyPath, "utf8")
		const fromSecretKey = Uint8Array.from(JSON.parse(secretKeyString))
		this.wallet = Keypair.fromSecretKey(fromSecretKey)
	}

	// -- 执行 SOL 转账
	async transfer(
		toAddress: string,
		lamports: number,
		config?: TransferConfig,
	): Promise<TransferResult> {
		try {
			// -- 准备指令列表
			const instructions: TransactionInstruction[] = []

			// -- 设置CU价格（如果提供）
			if (config?.computeUnitPrice) {
				instructions.push(
					ComputeBudgetProgram.setComputeUnitPrice({
						microLamports: config.computeUnitPrice,
					}),
				)
			}

			// -- 设置 CU 限制（如果提供）
			if (config?.computeUnitLimit) {
				instructions.push(
					ComputeBudgetProgram.setComputeUnitLimit({
						units: config.computeUnitLimit,
					}),
				)
			}

			// -- 添加转账指令
			instructions.push(
				SystemProgram.transfer({
					fromPubkey: this.wallet.publicKey,
					toPubkey: new PublicKey(toAddress),
					lamports,
				}),
			)

			// -- 获取最新的区块哈希
			const latestBlockhash =
				await this.connection.getLatestBlockhash(COMMITMENT)
			logger.debug(`获取最新区块哈希: ${latestBlockhash.blockhash}`)

			// -- 创建交易消息
			const messageV0 = new TransactionMessage({
				payerKey: this.wallet.publicKey,
				recentBlockhash: latestBlockhash.blockhash,
				instructions,
			}).compileToV0Message()

			// -- 创建版本化交易
			const transaction = new VersionedTransaction(messageV0)

			// -- 模拟交易
			const simulateResult =
				await this.connection.simulateTransaction(transaction)
			logger.debug(`模拟交易结果: ${JSON.stringify(simulateResult)}`)

			// -- 如果模拟成功，签名并发送交易
			if (simulateResult.value.err) {
				logger.error(
					`交易模拟失败: ${JSON.stringify(simulateResult.value.err)}`,
				)
				throw new Error(
					`交易模拟失败: ${JSON.stringify(simulateResult.value.err)}`,
				)
			}

			// -- 签名交易
			transaction.sign([this.wallet])

			// -- 发送交易
			const signature = await this.connection.sendTransaction(transaction, {
				maxRetries: 3,
			})
			logger.debug(`交易签名: ${signature}`)

			// -- 确认交易
			const confirmation = await this.connection.confirmTransaction({
				signature,
				blockhash: latestBlockhash.blockhash,
				lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
			})
			logger.debug(`交易确认: ${JSON.stringify(confirmation)}`)

			if (confirmation.value.err) {
				logger.error(`交易未确认: ${JSON.stringify(confirmation)}`)
				throw new Error("交易未确认")
			}

			return {
				success: true,
				signature,
				simulateResult: simulateResult.value,
				url: `https://solscan.io/tx/${signature}`,
			}
		} catch (error) {
			logger.error("转账失败:")
			logger.error(error as LogMessage)
			return {
				success: false,
				error: error instanceof Error ? error.message : String(error),
			}
		}
	}

	// -- 批量转账
	async batchTransfer(
		toAddress: string,
		lamports: number,
		count: number,
		config?: TransferConfig,
	): Promise<TransferResult[]> {
		const results: TransferResult[] = []

		logger.info(`开始批量转账，共 ${count} 笔交易`)

		for (let i = 0; i < count; i++) {
			logger.info(`执行第 ${i + 1}/${count} 笔转账`)
			const result = await this.transfer(toAddress, lamports, config)
			results.push(result)

			// -- 添加延迟以避免触发速率限制
			if (i < count - 1) {
				await new Promise((resolve) => setTimeout(resolve, 1000))
			}
		}

		// -- 统计结果
		const succeeded = results.filter((r) => r.success).length
		const failed = count - succeeded

		logger.success(`批量转账完成，成功: ${succeeded}，失败: ${failed}`)

		return results
	}
}
