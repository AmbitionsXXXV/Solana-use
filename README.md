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

### 3. 工具包 [`toolkits`](./toolkits)

提供了一系列可复用的工具和库：

- **Solana 工具包**
  - 账户管理
  - 交易处理
  - 日志监控
  - 错误处理

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

```
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

- 确保已安装 Solana CLI 工具
- 使用前请仔细阅读配置说明
- 测试网操作请使用测试账户

## 许可证

MIT
