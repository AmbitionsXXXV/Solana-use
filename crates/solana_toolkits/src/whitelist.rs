use std::collections::HashSet;

/// -- 代币白名单管理器
/// 用于管理不应该被关闭的代币账户的白名单
#[derive(Debug, Default)]
pub struct TokenWhitelist {
    symbols: HashSet<String>, // -- 代币符号白名单
    mints: HashSet<String>,   // -- 代币 Mint 地址白名单
}

impl TokenWhitelist {
    /// -- 创建新的白名单管理器
    pub fn new() -> Self {
        Self {
            symbols: HashSet::new(),
            mints: HashSet::new(),
        }
    }

    /// -- 添加代币符号到白名单
    pub fn add_symbol(&mut self, symbol: &str) {
        self.symbols.insert(symbol.to_uppercase());
    }

    /// -- 添加 Mint 地址到白名单
    pub fn add_mint(&mut self, mint: &str) {
        self.mints.insert(mint.to_string());
    }

    /// -- 检查代币是否在白名单中
    pub fn is_whitelisted(&self, symbol: &str, mint: &str) -> bool {
        self.symbols.contains(&symbol.to_uppercase()) || self.mints.contains(mint)
    }
}