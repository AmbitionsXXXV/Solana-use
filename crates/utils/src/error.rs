use std::error::Error;
use thiserror::Error;

/// -- 代币账户管理错误类型
///
/// 封装了代币账户管理过程中可能遇到的各种错误：
/// - RPC 相关错误
/// - 账户操作错误
/// - 数据解析错误
/// - 交易执行错误
/// - IO 和系统错误
#[derive(Debug, Error)]
pub enum TokenAccountError {
    /// RPC 客户端错误
    #[error("RPC 错误: {0}")]
    RpcError(#[from] solana_client::client_error::ClientError),

    /// 密钥格式无效
    #[error("无效的密钥格式")]
    InvalidKeyFormat,

    /// 账户数据解析错误
    #[error("账户解析错误: {0}")]
    AccountParseError(String),

    /// 账户余额非零错误
    #[error("账户余额不为 0: {0}")]
    NonZeroBalance(u64),

    /// 交易执行错误
    #[error("交易错误: {0}")]
    TransactionError(String),

    /// IO 操作错误
    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON 解析错误
    #[error("JSON 解析错误: {0}")]
    JsonError(#[from] serde_json::Error),

    /// SPL Token 操作错误
    #[error("SPL Token 错误: {0}")]
    SplTokenError(#[from] spl_token::error::TokenError),

    /// Solana 程序执行错误
    #[error("程序错误: {0}")]
    ProgramError(#[from] solana_sdk::program_error::ProgramError),

    /// 其他未分类错误
    #[error("其他错误: {0}")]
    Other(String),
}

impl From<Box<dyn Error>> for TokenAccountError {
    fn from(error: Box<dyn Error>) -> Self {
        TokenAccountError::Other(error.to_string())
    }
}

impl From<anyhow::Error> for TokenAccountError {
    fn from(error: anyhow::Error) -> Self {
        TokenAccountError::Other(error.to_string())
    }
}

/// -- 自定义 Result 类型
///
/// 用于代币账户管理操作的统一返回类型
pub type TokenAccountResult<T> = Result<T, TokenAccountError>;
