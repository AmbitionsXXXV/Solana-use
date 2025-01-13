// -- 网络连接配置
export const SOLANA_RPC_URL =
    process.env.HELIUS_RPC_URL ?? "https://api.mainnet-beta.solana.com"
export const COMMITMENT = "confirmed" as const

// -- Raydium V4 程序ID
export const RAYDIUM_V4 = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"

// -- 默认计算单元配置
export const DEFAULT_COMPUTE_UNIT_PRICE = 5 // microLamports
export const DEFAULT_COMPUTE_UNIT_LIMIT = 500_000
