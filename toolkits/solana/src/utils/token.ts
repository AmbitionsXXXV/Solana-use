import { Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import { TOKEN_PROGRAM_ID, getAccount } from "@solana/spl-token"
import { type LogMessage, logger } from "@xxhh/toolkits-logger"
import { COMMITMENT, SOLANA_RPC_URL } from "../constant"
import type { TokenAccount, TokenAccountsResult } from "../types/token"

// -- 查询可关闭的代币账户
export async function getClosableTokenAccounts(
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
