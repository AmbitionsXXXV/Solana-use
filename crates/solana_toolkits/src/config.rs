use solana_sdk::commitment_config::CommitmentConfig;
use std::time::Duration;

/// -- 代币账户管理配置
///
/// 用于配置代币账户管理器的各项参数
#[derive(Debug, Clone)]
pub struct TokenAccountConfig {
    /// Solana 网络提交配置
    pub commitment: CommitmentConfig,
    /// 批处理操作间隔时间
    pub batch_delay: Duration,
    /// 最大重试次数
    pub max_retries: u32,
    /// 重试间隔时间
    pub retry_delay: Duration,
}

impl Default for TokenAccountConfig {
    fn default() -> Self {
        Self {
            commitment: CommitmentConfig::confirmed(),
            batch_delay: Duration::from_millis(2000),
            max_retries: 3,
            retry_delay: Duration::from_millis(1000),
        }
    }
}
