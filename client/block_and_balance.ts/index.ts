import { Connection, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js'
import 'dotenv/config'
import process from 'node:process'

// 运行：
// pnpx esrun client/block_and_balance.ts/index.ts <your mainnet wallet address>

// 创建到 Solana 网络的连接，这里以主网为例
const connection = new Connection('https://api.mainnet-beta.solana.com', 'confirmed')

/**
 * 获取最新的区块信息和指定账户的余额
 */
async function getLatestBlockInfo() {
  try {
    // 获取最新的区块高度
    const slot = await connection.getSlot()
    console.log(`最新的区块高度（Slot）: ${slot}`)

    // 获取区块的详细信息
    const blockInfo = await connection.getBlock(slot, {
      maxSupportedTransactionVersion: 0,
    })
    console.log('区块信息:', blockInfo)
  } catch (error) {
    console.error('获取区块信息时出错:', error)
  }
}

/**
 * 获取指定账户的余额
 * @param publicKeyString 要查询余额的账户公钥地址
 */
async function getAccountBalance(publicKeyString: string) {
  try {
    // 将提供的公钥字符串转换为 PublicKey 对象
    const publicKey = new PublicKey(publicKeyString)

    // 查询余额
    const balance = await connection.getBalance(publicKey)
    console.log(`账户 ${publicKeyString} 的余额: ${balance / LAMPORTS_PER_SOL} SOL`)
  } catch (error) {
    console.error('查询账户余额时出错:', error)
  }
}

// 调用函数获取最新的区块信息
getLatestBlockInfo()

// 检查是否提供了命令行参数
if (process.argv.length < 3) {
  console.log('请提供要查询余额的账户公钥地址作为命令行参数。')
  process.exit(1) // 退出程序
}

// 使用命令行参数作为要查询余额的账户公钥
const publicKeyString = process.argv[2]
getAccountBalance(publicKeyString)
