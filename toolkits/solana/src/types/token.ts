// -- 代币账户接口定义
export interface TokenAccount {
    address: string
    mint: string
    rentLamports: number
    rentSol: number
}

// -- 代币账户查询结果接口
export interface TokenAccountsResult {
    totalAccounts: number
    closableAccounts: number
    accounts: TokenAccount[]
    totalRentLamports: number
    totalRentSol: number
}

// -- 账户关闭结果接口
export interface ClosureResult {
    success: boolean
    signature?: string
    error?: string
    accountAddress: string
    rentRecovered: number
}
