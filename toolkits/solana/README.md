# Solana 工具包

这是一个用于 Solana 区块链的工具包，提供了一系列实用功能，包括代币账户管理、SOL 转账等操作。

## 功能特性

### 1. 代币账户管理
- 查询钱包下的所有代币账户
- 筛选可关闭的零余额账户
- 批量关闭代币账户，回收租金
- 详细的租金统计和操作日志

### 2. SOL 转账
- 支持单笔和批量转账
- 使用 v0 版本化交易（Versioned Transactions）
- 支持自定义计算单元（Compute Units）配置
- 完整的交易生命周期管理（模拟、签名、确认）
- 详细的交易状态和错误日志

## 安装

```bash
pnpm add @xxhh/toolkits-solana
```

## 使用示例

### 代币账户管理

```typescript
import { TokenAccountManager, getClosableTokenAccounts } from '@xxhh/toolkits-solana'
import { logger } from '@xxhh/toolkits-logger'

// 查询可关闭的代币账户
const result = await getClosableTokenAccounts(walletAddress)
logger.info(`找到 ${result.closableAccounts} 个可关闭账户`)
logger.info(`总计可返还租金: ${result.totalRentSol} SOL`)

// 批量关闭账户
const manager = new TokenAccountManager(walletKeyPath)
await manager.batchCloseAccounts(result.accounts)
```

### SOL 转账

```typescript
import { 
    TransferManager, 
    DEFAULT_COMPUTE_UNIT_PRICE,
    DEFAULT_COMPUTE_UNIT_LIMIT 
} from '@xxhh/toolkits-solana'

// 创建转账管理器
const manager = new TransferManager(walletKeyPath, SOLANA_RPC_URL)

// 单笔转账
const result = await manager.transfer(
    "接收地址",
    1000, // lamports
    {
        computeUnitPrice: DEFAULT_COMPUTE_UNIT_PRICE,
        computeUnitLimit: DEFAULT_COMPUTE_UNIT_LIMIT,
    }
)

if (result.success) {
    console.log(`转账成功，交易签名: ${result.signature}`)
    console.log(`交易链接: ${result.url}`)
}

// 批量转账
const results = await manager.batchTransfer(
    "接收地址",
    1000, // lamports
    5, // 转账次数
    {
        computeUnitPrice: DEFAULT_COMPUTE_UNIT_PRICE,
        computeUnitLimit: DEFAULT_COMPUTE_UNIT_LIMIT,
    }
)
```

## 配置说明

### 环境变量
- `HELIUS_RPC_URL`: Helius RPC 节点地址（可选）
- `WALLET_PATH`: 钱包私钥文件路径

### 常量配置
- `DEFAULT_COMPUTE_UNIT_PRICE`: 默认计算单元价格（5 microLamports）
- `DEFAULT_COMPUTE_UNIT_LIMIT`: 默认计算单元限制（500,000）

## 技术特性

### 版本化交易（v0）
- 支持地址查找表（Address Lookup Tables）
- 账户数量限制提升至 64 个
- 更高效的交易处理

### 错误处理
- 完整的交易生命周期监控
- 详细的错误信息和状态日志
- 交易模拟验证

### 性能优化
- 批量操作支持
- 自动重试机制
- 速率限制保护

## 开发说明

### 构建
```bash
pnpm build
```

### 测试
```bash
pnpm test
```

### 代码风格
- 使用 TypeScript
- 使用 Biome 进行代码规范和格式化

## 许可证

MIT
