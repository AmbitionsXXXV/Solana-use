import { logger } from "@etc/toolkits-logger"
import { RAYDIUM_V4 } from "./constant"
import { Connection, PublicKey, clusterApiUrl } from "@solana/web3.js"

export function subRay() {
	const raydiumV4PublicKey = new PublicKey(RAYDIUM_V4)

	// 创建一个到 Solana 主网 beta 的连接
	// 'confirmed' 表示我们等待交易被确认
	const connection = new Connection(clusterApiUrl("mainnet-beta"), "confirmed")

	// 调用 onLogs hook 来监控 Raydium V4 合约的日志
	connection.onLogs(raydiumV4PublicKey, ({ logs, err, signature }) => {
		// 如果有错误发生，立即返回
		if (err) return

		// 检查日志中是否有包含 "initialize2" 的条目
		if (logs?.some((log) => log.includes("initialize2"))) {
			// 如果找到了 "initialize2"，打印出交易的签名
			logger.info(`Signature for initialize2 '${signature}':`)
		}
	})
}
