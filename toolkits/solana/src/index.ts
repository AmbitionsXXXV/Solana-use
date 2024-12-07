import { TOKEN_PROGRAM_ID } from "@solana/spl-token"
import { Connection, PublicKey } from "@solana/web3.js"
import { type LogMessage, logger } from "@xxhh/toolkits-logger"

// -- 定义返回类型接口
interface TokenAccount {
	address: string
	mint: string
}

interface TokenAccountsResult {
	totalAccounts: number
	closableAccounts: number
	accounts: TokenAccount[]
}

// -- 网络连接配置
const SOLANA_RPC_URL = "https://api.mainnet-beta.solana.com"
const COMMITMENT = "confirmed"

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

		const closableAccounts = tokenAccounts.value
			.filter((account) => {
				const tokenAmount = account.account.data.parsed.info.tokenAmount
				return tokenAmount.uiAmount === 0
			})
			.map((account) => ({
				address: account.pubkey.toString(),
				mint: account.account.data.parsed.info.mint,
			}))

		// -- 使用新的日志模块
		logger.info(`总代币账户数量: ${tokenAccounts.value.length}`)
		logger.info(`可关闭账户数量: ${closableAccounts.length}`)

		closableAccounts.forEach(({ address, mint }, index) => {
			logger.debug(`\n可关闭账户 ${index + 1}:`)
			logger.debug(`账户地址: ${address}`)
			logger.debug(`代币 Mint: ${mint}`)
		})

		return {
			totalAccounts: tokenAccounts.value.length,
			closableAccounts: closableAccounts.length,
			accounts: closableAccounts,
		}
	} catch (error) {
		logger.error("查询代币账户失败:")
		logger.error(error as LogMessage)
		throw new Error(
			`查询代币账户失败: ${error instanceof Error ? error.message : String(error)}`,
		)
	}
}

// 使用示例
async function main() {
	try {
		logger.info("开始查询可关闭的代币账户...")
		const result = await getClosableTokenAccounts("钱包地址")
		logger.success("查询完成")
		logger.table(result.accounts)
	} catch (error) {
		logger.error("执行失败:")
		logger.error(error as LogMessage)
	}
}

main()
