import type { SimulatedTransactionResponse } from "@solana/web3.js"

// -- 传输配置接口
export interface TransferConfig {
	computeUnitPrice?: number
	computeUnitLimit?: number
}

// -- 传输结果接口
export interface TransferResult {
	success: boolean
	signature?: string
	error?: string
	simulateResult?: SimulatedTransactionResponse
	url?: string
}
