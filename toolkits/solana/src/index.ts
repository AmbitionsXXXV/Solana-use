import {
	TOKEN_PROGRAM_ID,
	createCloseAccountInstruction,
	getAccount,
} from "@solana/spl-token"
import {
	Connection,
	Keypair,
	LAMPORTS_PER_SOL,
	PublicKey,
	Transaction,
	sendAndConfirmTransaction,
} from "@solana/web3.js"
import { type LogMessage, logger } from "@xxhh/toolkits-logger"
import bs58 from "bs58"
import { config } from "dotenv"
import * as fs from "node:fs"

config({ path: ".env" })

// -- 定义返回类型接口
interface TokenAccount {
	address: string
	mint: string
	rentLamports: number
	rentSol: number
}

interface TokenAccountsResult {
	totalAccounts: number
	closableAccounts: number
	accounts: TokenAccount[]
	totalRentLamports: number
	totalRentSol: number
}

interface ClosureResult {
	success: boolean
	signature?: string
	error?: string
	accountAddress: string
	rentRecovered: number
}

// -- 网络连接配置
const SOLANA_RPC_URL = "https://api.mainnet-beta.solana.com"
const COMMITMENT = "confirmed"

class TokenAccountManager {
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
				rentRecovered: rentBefore / LAMPORTS_PER_SOL,
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
		if (accounts.length === 0) {
			logger.warn("没有找到可关闭的账户")
			return
		}

		// -- 获取执行前钱包余额
		const balanceBefore = await this.connection.getBalance(
			this.wallet.publicKey,
		)
		const balanceBeforeSOL = balanceBefore / LAMPORTS_PER_SOL

		let totalRentRecovered = 0
		let successCount = 0
		let failCount = 0

		// -- 分批处理账户
		for (let i = 0; i < accounts.length; i += batchSize) {
			const batch = accounts.slice(i, i + batchSize)
			logger.info(`\n处理第 ${i / batchSize + 1} 批, 共 ${batch.length} 个账户`)

			// -- 并行处理当前批次的账户
			const results = await Promise.all(
				batch.map((account) =>
					this.closeAccount(new PublicKey(account.address)),
				),
			)

			// -- 统计结果
			for (const result of results) {
				if (result.success) {
					successCount++
					totalRentRecovered += result.rentRecovered
					logger.success(`成功关闭账户: ${result.accountAddress}`)
					logger.info(`交易签名: ${result.signature}`)
					logger.info(`回收租金: ${result.rentRecovered} SOL`)
				} else {
					failCount++
					logger.error(`关闭失败: ${result.accountAddress}`)
					logger.error(`错误信息: ${result.error}`)
				}
			}

			// -- 批次间延时，避免请求过于频繁
			if (i + batchSize < accounts.length) {
				await new Promise((resolve) => setTimeout(resolve, 2000))
			}
		}

		// -- 获取执行后钱包余额
		const balanceAfter = await this.connection.getBalance(this.wallet.publicKey)
		const balanceAfterSOL = balanceAfter / LAMPORTS_PER_SOL
		const actualRecovered = balanceAfterSOL - balanceBeforeSOL

		// -- 获取 GAS 总消耗
		const gasConsumed = actualRecovered - totalRentRecovered

		// -- 输出最终统计结果
		logger.success("\n====== 处理完成 ======")
		logger.info(`执行前钱包余额: ${balanceBeforeSOL.toFixed(8)} SOL`)
		logger.info(`执行后钱包余额: ${balanceAfterSOL.toFixed(8)} SOL`)
		logger.success(`实际增加余额: ${actualRecovered.toFixed(8)} SOL`)
		logger.info(`成功关闭: ${successCount} 个账户`)
		logger.info(`失败数量: ${failCount} 个账户`)
		logger.success(`预计回收租金: ${totalRentRecovered.toFixed(8)} SOL`)
		logger.success(
			`GAS 消耗: ${gasConsumed} lamports, ${gasConsumed / LAMPORTS_PER_SOL} SOL`,
		)
	}
}

async function getClosableTokenAccounts(
	walletAddress: string,
): Promise<TokenAccountsResult> {
	if (!walletAddress) {
		throw new Error("钱包地址不能为空")
	}

	const connection = new Connection(SOLANA_RPC_URL, COMMITMENT)
	const pubKey = new PublicKey(walletAddress)

	try {
		const tokenAccounts = await connection.getParsedTokenAccountsByOwner(
			pubKey,
			{
				programId: TOKEN_PROGRAM_ID,
			},
		)

		const accountInfos = await Promise.all(
			tokenAccounts.value
				.filter((account) => {
					const tokenAmount = account.account.data.parsed.info.tokenAmount
					return tokenAmount.uiAmount === 0
				})
				.map(async (account) => {
					const accountInfo = await connection.getAccountInfo(account.pubkey)
					const rentLamports = accountInfo?.lamports || 0
					return {
						address: account.pubkey.toString(),
						mint: account.account.data.parsed.info.mint,
						rentLamports,
						rentSol: rentLamports / LAMPORTS_PER_SOL,
					}
				}),
		)

		const totalRentLamports = accountInfos.reduce(
			(sum, account) => sum + account.rentLamports,
			0,
		)
		const totalRentSol = totalRentLamports / LAMPORTS_PER_SOL

		logger.info(`总代币账户数量: ${tokenAccounts.value.length}`)
		logger.info(`可关闭账户数量: ${accountInfos.length}`)
		logger.success({
			totalRentLamports,
			totalRentSol: totalRentSol.toFixed(8),
		})

		accountInfos.forEach((account, index) => {
			logger.debug(`\n可关闭账户 ${index + 1}:`)
			logger.debug(`账户地址: ${account.address}`)
			logger.debug(`代币 Mint: ${account.mint}`)
			logger.debug(
				`可返还租金: ${account.rentLamports} lamports (${account.rentSol.toFixed(
					8,
				)} SOL)`,
			)
		})

		return {
			totalAccounts: tokenAccounts.value.length,
			closableAccounts: accountInfos.length,
			accounts: accountInfos,
			totalRentLamports,
			totalRentSol,
		}
	} catch (error) {
		logger.error("查询代币账户失败:")
		logger.error(error as LogMessage)
		throw new Error(
			`查询代币账户失败: ${error instanceof Error ? error.message : String(error)}`,
		)
	}
}

// -- 使用示例
async function main() {
	try {
		const walletPath = process.env.WALLET_PATH
		if (!walletPath) {
			logger.warn("未提供钱包私钥文件路径（WALLET_PATH），无法执行操作")
			return
		}

		// -- 创建 TokenAccountManager 实例并获取钱包地址
		const manager = new TokenAccountManager(walletPath, SOLANA_RPC_URL)
		const walletAddress = manager.wallet.publicKey.toString()

		logger.info(`使用钱包地址: ${walletAddress}`)
		logger.info("开始查询可关闭的代币账户...")

		const result = await getClosableTokenAccounts(walletAddress)
		logger.success("查询完成")
		logger.table(result.accounts)
		logger.success(
			`总计可返还租金: ${result.totalRentLamports} lamports (${result.totalRentSol.toFixed(8)} SOL)`,
		)

		logger.info("开始执行账户关闭操作...")
		await manager.batchCloseAccounts(result.accounts)
	} catch (error) {
		logger.error("执行失败:")
		logger.error(error as LogMessage)
	}
}

main()
