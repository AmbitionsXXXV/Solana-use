use std::collections::HashSet;

/// -- 代币白名单管理器
/// 用于管理不应该被关闭的代币账户的白名单
#[derive(Debug, Default)]
pub struct TokenWhitelist {
    symbols: HashSet<String>, // -- 代币符号白名单
    mints: HashSet<String>,   // -- 代币 Mint 地址白名单
    user_added: bool,         // -- 是否已添加用户自定义白名单
    merge_default: bool,      // -- 是否合并默认白名单
}

impl TokenWhitelist {
    /// -- 默认白名单符号
    const DEFAULT_SYMBOLS: [&'static str; 3] = ["USDC", "USDT", "SOL"]; // -- 可以根据需要添加更多

    /// -- 创建新的白名单管理器
    ///
    /// # 参数
    ///
    /// * `merge_default` - 是否合并用户添加的白名单和默认白名单, 默认为 false
    pub fn new(merge_default: Option<bool>) -> Self {
        Self {
            symbols: HashSet::new(),
            mints: HashSet::new(),
            user_added: false,
            merge_default: merge_default.unwrap_or(false), // -- 设置是否合并, 默认为 false
        }
    }

    /// -- 添加代币符号到白名单（单个添加）
    pub fn add_symbol(&mut self, symbol: &str) {
        self.symbols.insert(symbol.to_uppercase());
        self.user_added = true; // -- 标记已添加用户自定义白名单
    }

    /// -- 添加 Mint 地址到白名单（单个添加）
    pub fn add_mint(&mut self, mint: &str) {
        self.mints.insert(mint.to_string());
        self.user_added = true; // -- 标记已添加用户自定义白名单
    }

    /// -- 批量添加代币符号到白名单
    pub fn add_symbols(&mut self, symbols: &[&str]) {
        for &symbol in symbols {
            self.symbols.insert(symbol.to_uppercase());
        }
        self.user_added = true; // -- 标记已添加用户自定义白名单
    }

    /// -- 批量添加 Mint 地址到白名单
    pub fn add_mints(&mut self, mints: &[&str]) {
        self.mints.extend(mints.iter().map(|s| s.to_string()));
        self.user_added = true; // -- 标记已添加用户自定义白名单
    }

    /// -- 检查代币是否在白名单中
    pub fn is_whitelisted(&self, symbol: &str, mint: &str) -> bool {
        let symbol_uppercase = symbol.to_uppercase();
        let mut is_in_whitelist =
            self.symbols.contains(&symbol_uppercase) || self.mints.contains(mint);

        // -- 如果启用合并或尚未添加用户自定义白名单，则检查默认白名单
        if self.merge_default || !self.user_added {
            is_in_whitelist = is_in_whitelist
                || Self::DEFAULT_SYMBOLS
                    .iter()
                    .any(|&default_symbol| default_symbol.to_uppercase() == symbol_uppercase)
                || self.mints.contains(mint); // -- 默认白名单不包含 mint, 所以这里不需要特殊处理
        }

        is_in_whitelist
    }

    /// -- 设置是否合并默认白名单和用户添加的白名单
    pub fn set_merge_default(&mut self, merge_default: bool) {
        self.merge_default = merge_default;
    }
}
