import { TOKEN_PROGRAM_ID } from "@solana/spl-token"
import { Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import { type LogMessage, logger } from "@xxhh/toolkits-logger"

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
					4,
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

// 使用示例
async function main() {
	try {
		logger.info("开始查询可关闭的代币账户...")
		const result = await getClosableTokenAccounts("钱包地址")
		logger.success("查询完成")
		logger.table(result.accounts)
		logger.success(
			`总计可返还租金: ${result.totalRentLamports} lamports (${result.totalRentSol.toFixed(
				8,
			)} SOL)`,
		)
	} catch (error) {
		logger.error("执行失败:")
		logger.error(error as LogMessage)
	}
}

main()
