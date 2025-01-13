import { type LogMessage, logger } from "@xxhh/toolkits-logger"
import { config } from "dotenv"
import { subRay } from "./sub_ray"

config({ path: ".env" })

// -- 使用示例
async function main() {
	try {
		// const walletPath = process.env.WALLET_PATH
		// if (!walletPath) {
		// 	logger.warn("未提供钱包私钥文件路径（WALLET_PATH），无法执行操作")
		// 	return
		// }

		// // -- 创建 TokenAccountManager 实例并获取钱包地址
		// const manager = new TokenAccountManager(walletPath, SOLANA_RPC_URL)
		// const walletAddress = manager.wallet.publicKey.toString()

		// logger.info(`使用钱包地址: ${walletAddress}`)
		// logger.info("开始查询可关闭的代币账户...")

		// const result = await getClosableTokenAccounts(walletAddress)
		// logger.success("查询完成")
		// logger.table(result.accounts)
		// logger.success(
		// 	`总计可返还租金: ${result.totalRentLamports} lamports (${result.totalRentSol.toFixed(8)} SOL)`,
		// )

		// logger.info("开始执行账户关闭操作...")
		// await manager.batchCloseAccounts(result.accounts)

		// ------------------------------------------------
		// -- 使用示例：Raydium V4 合约日志监控
		subRay()
	} catch (error) {
		logger.error("执行失败:")
		logger.error(error as LogMessage)
	}
}

main()
