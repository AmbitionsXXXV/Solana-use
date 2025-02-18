/// -- 代币账户信息结构体
/// 存储单个代币账户的基本信息，包括地址、Mint、租金等
#[derive(Debug)]
pub struct TokenAccountInfo {
    pub address: String,    // -- 账户地址
    pub mint: String,       // -- 代币的 Mint 地址
    pub rent_lamports: u64, // -- 租金（以 lamports 为单位）
    pub rent_sol: f64,      // -- 租金（以 SOL 为单位）
    pub symbol: String,     // -- 代币符号
}

/// -- 零值代币账户信息结构体
#[derive(Debug)]
pub struct ZeroValueTokenInfo {
    pub address: String,    // -- 账户地址
    pub mint: String,       // -- 代币的 Mint 地址
    pub balance: u64,       // -- 代币余额
    pub rent_lamports: u64, // -- 租金（以 lamports 为单位）
    pub rent_sol: f64,      // -- 租金（以 SOL 为单位）
    pub symbol: String,     // -- 代币符号
}

/// -- 代币账户查询结果结构体
/// 包含查询到的所有代币账户统计信息
#[derive(Debug)]
pub struct TokenAccountsResult {
    pub total_accounts: usize,                             // -- 总账户数量
    pub closable_accounts: usize,                          // -- 可关闭的账户数量（余额为 0）
    pub zero_value_accounts: usize,                        // -- 零值代币账户数量
    pub accounts: Vec<TokenAccountInfo>,                   // -- 可关闭账户列表（余额为 0）
    pub zero_value_accounts_list: Vec<ZeroValueTokenInfo>, // -- 零值代币账户列表
    pub total_rent_lamports: u64,                          // -- 总租金（以 lamports 为单位）
    pub total_rent_sol: f64,                               // -- 总租金（以 SOL 为单位）
}

/// -- 账户关闭结果结构体
/// 记录单个账户关闭操作的结果
#[derive(Debug)]
pub struct ClosureResult {
    pub success: bool,             // -- 操作是否成功
    pub signature: Option<String>, // -- 成功时的交易签名
    pub error: Option<String>,     // -- 失败时的错误信息
    pub account_address: String,   // -- 被关闭的账户地址
    pub rent_recovered: f64,       // -- 回收的租金数量（以 SOL 为单位）
}

/// -- 代币账户详细信息结构体
/// 存储代币账户的完整信息
#[derive(Debug)]
pub struct TokenAccountDetails {
    pub pubkey: String,     // -- 账户公钥
    pub balance: u64,       // -- 账户余额
    pub rent_lamports: u64, // -- 租金（以 lamports 为单位）
    pub rent_sol: f64,      // -- 租金（以 SOL 为单位）
    pub mint: String,       // -- 代币的 Mint 地址
    pub owner: String,      // -- 账户所有者地址
}

/// -- 销毁代币并回收账户结果结构体
#[derive(Debug)]
pub struct BurnAndCloseResult {
    pub success: bool,                   // -- 操作是否成功
    pub burn_signature: Option<String>,  // -- 销毁代币的交易签名
    pub close_signature: Option<String>, // -- 关闭账户的交易签名
    pub error: Option<String>,           // -- 失败时的错误信息
    pub account_address: String,         // -- 被操作的账户地址
    pub burned_amount: u64,              // -- 销毁的代币数量
    pub rent_recovered: f64,             // -- 回收的租金数量（以 SOL 为单位）
}
