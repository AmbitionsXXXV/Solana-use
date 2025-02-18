# Solana 学习与开发工具集

这是一个关于 Solana 区块链开发的学习和工具集合项目，包含了多个实用的示例和工具包。

## 项目结构

### 1. 客户端工具 [`client`](./client)

提供了多个实用的客户端工具和示例：

- **账户管理**
  - 创建 Solana 账户
  - 批量创建代币账户
  - 加载和管理密钥对
  - 查询和关闭零余额账户

- **代币操作**
  - USDC 转账示例
  - SOL 转账（支持 v0 交易）
  - 批量转账功能

### 2. Solana 程序 [`solana-program`](./solana-program)

包含了基础的 Solana 智能合约示例：

- Hello World 程序
- 基本的链上交互
- 程序部署示例

### 3. Solana 工具包 [`solana_toolkits`](./crates/solana_toolkits)

提供了一系列 Solana 账户管理和代币操作的工具：

#### 主要功能

- **代币账户管理**
  - 查询账户详情
  - 关闭零余额账户
  - 批量关闭账户
  - 销毁代币并回收账户
  - 批量销毁零值代币账户

- **白名单管理**
  - 默认白名单（USDC、USDT、SOL）
  - 自定义代币白名单
  - 支持通过代币符号和 Mint 地址添加
  - 批量添加白名单功能

#### 使用示例

```rust
// 创建账户管理器实例
let mut manager = TokenAccountManager::new("wallet.json")?;

// 白名单管理
manager.set_merge_default_whitelist(true);  // 启用默认白名单
manager.add_symbol_to_whitelist("RAY");     // 添加单个代币符号
manager.add_symbols_to_whitelist(&["BONK", "SAMO"]);  // 批量添加代币符号
manager.add_mint_to_whitelist("mint_address");  // 添加 Mint 地址
manager.add_mints_to_whitelist(&["mint1", "mint2"]);  // 批量添加 Mint 地址

// 查询可关闭的账户
let accounts = manager.get_closeable_accounts().await?;

// 关闭单个账户
let result = manager.close_account(&account_pubkey).await;

// 批量关闭账户
manager.batch_close_accounts(&accounts.accounts, 5, true).await?;

// 销毁代币并关闭账户
let result = manager.burn_and_close_account(&account_pubkey).await;

// 批量销毁零值代币并关闭账户
manager.batch_burn_and_close_zero_value_accounts(&accounts.zero_value_accounts_list, 5).await?;
```

#### 功能特点

- **安全性**
  - 白名单保护机制
  - 账户余额检查
  - 错误处理和日志记录

- **批量操作**
  - 支持批量关闭账户
  - 可配置批次大小
  - 自动延时防止限流

- **资源回收**
  - 自动回收租金
  - 支持销毁代币
  - 详细的操作统计

## 快速开始

### 1. 环境配置

```bash
# 安装依赖
pnpm install

# 配置环境变量
cp .env.example .env
# 编辑 .env 文件，添加必要的配置
```

### 2. 运行 Solana 程序

具体查看各个 `example` 文件

### 3. 使用客户端工具

```bash
# 创建 Solana 账户
pnpm run create_account

# 批量创建代币账户
pnpm run batch_create_accounts

# USDC 转账
pnpm run transfer_usdc
```

## 开发指南

### 1. 项目结构说明

```bash
.
├── client/               # 客户端工具和示例
│   ├── create_account/   # 创建账户
│   ├── transfer_usdc/    # USDC 转账
│   └── ...
├── crates/               # Solana 智能合约
│   └── solana_toolkits/  # Solana 工具包-Rust
└── toolkits/             # 工具包
    ├── logger/           # 日志工具
    └── solana/           # Solana 工具包
```

### 2. 开发流程

1. 克隆项目
2. 安装依赖
3. 配置环境变量
4. 运行示例或开发新功能

### 3. 注意事项

- 确保已有 Rust 环境
- 确保已安装 Solana CLI 工具
- 使用前请仔细阅读配置说明
- 测试网操作请使用测试账户

## 许可证

MIT
