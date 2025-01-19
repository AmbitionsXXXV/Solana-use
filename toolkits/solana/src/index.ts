import { type LogMessage, logger } from "@etc/toolkits-logger"
import { config } from "dotenv"
import { subRay } from "./sub_ray"
import { TokenAccountManager } from "./services/token-account"
import { TransferManager } from "./services/transfer"
import { getClosableTokenAccounts } from "./utils/token"
import type {
	TokenAccount,
	TokenAccountsResult,
	ClosureResult,
} from "./types/token"
import type { TransferConfig, TransferResult } from "./types/transfer"
import {
	SOLANA_RPC_URL,
	DEFAULT_COMPUTE_UNIT_LIMIT,
	DEFAULT_COMPUTE_UNIT_PRICE,
} from "./constant"

config({ path: ".env" })

export {
	TokenAccountManager,
	TransferManager,
	getClosableTokenAccounts,
	type TokenAccount,
	type TokenAccountsResult,
	type ClosureResult,
	type TransferConfig,
	type TransferResult,
	SOLANA_RPC_URL,
	DEFAULT_COMPUTE_UNIT_LIMIT,
	DEFAULT_COMPUTE_UNIT_PRICE,
}

// -- 使用示例
async function main() {
	try {
		// -- 示例：Raydium V4 合约日志监控
		subRay()

		// -- 示例：SOL转账
		// const walletPath = process.env.WALLET_PATH
		// if (!walletPath) {
		//     logger.warn("未提供钱包私钥文件路径（WALLET_PATH），无法执行操作")
		//     return
		// }

		// const transferManager = new TransferManager(walletPath, SOLANA_RPC_URL)
		// const result = await transferManager.transfer(
		//     "buffaAJKmNLao65TDTUGq8oB9HgxkfPLGqPMFQapotJ",
		//     1000,
		//     {
		//         computeUnitPrice: DEFAULT_COMPUTE_UNIT_PRICE,
		//         computeUnitLimit: DEFAULT_COMPUTE_UNIT_LIMIT,
		//     },
		// )

		// if (result.success) {
		//     logger.success(`转账成功，交易签名: ${result.signature}`)
		//     logger.info(`交易链接: ${result.url}`)
		// } else {
		//     logger.error(`转账失败: ${result.error}`)
		// }
	} catch (error) {
		logger.error("执行失败:")
		logger.error(error as LogMessage)
	}
}

main()
