import {
	Connection,
	Keypair,
	PublicKey,
	Transaction,
	sendAndConfirmTransaction,
} from "@solana/web3.js"
import {
	TOKEN_PROGRAM_ID,
	createCloseAccountInstruction,
	getAccount,
} from "@solana/spl-token"
import { logger } from "@xxhh/toolkits-logger"
import bs58 from "bs58"
import * as fs from "node:fs"
import { COMMITMENT, SOLANA_RPC_URL } from "../constant"
import type { ClosureResult, TokenAccount } from "../types/token"

export class TokenAccountManager {
	private connection: Connection
	public wallet: Keypair

	constructor(walletKeyPath: string, endpoint: string = SOLANA_RPC_URL) {
		this.connection = new Connection(endpoint, COMMITMENT)
		// -- 从文件加载私钥
		const secretKeyString = JSON.parse(fs.readFileSync(walletKeyPath, "utf8"))
		// -- 转换 base58 字符串为 Uint8Array
		const secretKey = bs58.decode(secretKeyString)

		this.wallet = Keypair.fromSecretKey(secretKey)
	}

	// -- 关闭单个账户
	async closeAccount(accountPubkey: PublicKey): Promise<ClosureResult> {
		try {
			// -- 获取账户当前状态
			const accountInfo = await getAccount(this.connection, accountPubkey)

			// -- 再次确认余额为0
			if (accountInfo.amount > BigInt(0)) {
				return {
					success: false,
					error: "账户余额不为0，无法关闭",
					accountAddress: accountPubkey.toString(),
					rentRecovered: 0,
				}
			}

			// -- 创建关闭账户的指令
			const closeInstruction = createCloseAccountInstruction(
				accountPubkey,
				this.wallet.publicKey, // -- 租金接收地址
				this.wallet.publicKey, // -- 账户所有者
				[],
				TOKEN_PROGRAM_ID,
			)

			// -- 获取关闭前的账户租金
			const rentBefore = await this.connection.getBalance(accountPubkey)

			// -- 构建并发送交易
			const transaction = new Transaction().add(closeInstruction)
			const signature = await sendAndConfirmTransaction(
				this.connection,
				transaction,
				[this.wallet],
				{
					commitment: COMMITMENT,
					maxRetries: 3,
				},
			)

			return {
				success: true,
				signature,
				accountAddress: accountPubkey.toString(),
				rentRecovered: 0, // TODO: 计算实际返还的租金
			}
		} catch (error) {
			return {
				success: false,
				error: error instanceof Error ? error.message : String(error),
				accountAddress: accountPubkey.toString(),
				rentRecovered: 0,
			}
		}
	}

	// -- 批量关闭账户
	async batchCloseAccounts(accounts: TokenAccount[], batchSize = 8) {
		const total = accounts.length
		let processed = 0
		let succeeded = 0

		logger.info(`开始批量关闭账户，共 ${total} 个账户`)

		while (processed < total) {
			const batch = accounts.slice(processed, processed + batchSize)
			const promises = batch.map((account) =>
				this.closeAccount(new PublicKey(account.address)),
			)

			const results = await Promise.all(promises)
			const batchSucceeded = results.filter((r) => r.success).length

			processed += batch.length
			succeeded += batchSucceeded

			logger.info(
				`已处理: ${processed}/${total} (成功: ${succeeded}, 失败: ${
					processed - succeeded
				})`,
			)

			// -- 添加延迟以避免触发速率限制
			if (processed < total) {
				await new Promise((resolve) => setTimeout(resolve, 1000))
			}
		}

		logger.success(
			`批量关闭完成，共处理 ${total} 个账户 (成功: ${succeeded}, 失败: ${
				total - succeeded
			})`,
		)
	}
}
