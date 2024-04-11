import { getKeypairFromEnvironment } from '@solana-developers/helpers'
import {
  Connection,
  PublicKey,
  SystemProgram,
  Transaction,
  clusterApiUrl,
  sendAndConfirmTransaction,
} from '@solana/web3.js'
import 'dotenv/config'

// 运行：
// pnpx esrun client/transfer/index.ts <接收者公钥>

// 从命令行参数中获取接收者的公钥
const suppliedToPubkey = process.argv[2] || null

// 如果没有提供接收者公钥，则提示用户输入并退出程序
if (!suppliedToPubkey) {
  console.log(`Please provide a public key to send to`)
  process.exit(1)
}

// 从环境变量中获取发送者的密钥对
const senderKeypair = getKeypairFromEnvironment('SECRET_KEY')

// 打印发送者密钥对和接收者公钥，用于验证
console.log(`suppliedToPubkey: ${suppliedToPubkey}`, senderKeypair)

// 将提供的接收者公钥字符串转换为 PublicKey 对象
const toPubkey = new PublicKey(suppliedToPubkey)

// 创建到 Solana devnet 的连接
// const connection = new Connection('https://api.mainnet-beta.solana.com', 'confirmed')
const connection = new Connection(clusterApiUrl('devnet'), 'confirmed')

// 确认已加载发送者密钥对，接收者公钥，并且已连接到 Solana 网络
console.log(
  `✅ Loaded our own keypair, the destination public key, and connected to Solana`,
)

const balance = await connection.getBalance(senderKeypair.publicKey)

// 创建一个新的交易对象
const transaction = new Transaction()

// 定义要发送的 lamports 数量（1 SOL = 1,000,000,000 lamports）
// 转 0.02 个 sol
const LAMPORTS_TO_SEND = balance - 5000

/**
 * 使用 SystemProgram.transfer 创建转账指令。
 * SystemProgram.transfer 需要：
 * - 发送者账户的公钥
 * - 接收者账户的公钥
 * - 要发送的 SOL 数量，以 lamports 为单位
 * 这是一个用于执行 Solana 系统程序中标准 SOL 转账的简化方法。
 */
const sendSolInstruction = SystemProgram.transfer({
  fromPubkey: senderKeypair.publicKey,
  toPubkey,
  // 默认发送所有余额
  lamports: LAMPORTS_TO_SEND,
})

// 将转账指令添加到交易中
transaction.add(sendSolInstruction)

// 发送交易并等待确认，使用发送者的密钥对进行签名
const signature = await sendAndConfirmTransaction(connection, transaction, [
  senderKeypair,
])

// 打印转账成功的消息和交易签名
console.log(`💸 Finished! Sent ${LAMPORTS_TO_SEND} to the address ${toPubkey}. `)

console.log(
  `Balance: ${(await connection.getBalance(senderKeypair.publicKey)) / 1000000000} Sol`,
)
console.log(
  `receiver balance: ${(await connection.getBalance(toPubkey)) / 1000000000} Sol`,
)

console.log(`Transaction signature is https://explorer.solana.com/tx/${signature}`)

// How much SOL did the transfer take? What is this in USD?
// 转移的 SOL 数量是 5000 / 1,000,000,000 SOL。

// Can you find your transaction on https://explorer.solana.com? Remember we are using the devnet network.

// How long does the transfer take?

// What do you think "confirmed" means?
// type Commitment = 'processed' | 'confirmed' | 'finalized' | 'recent' | 'single' | 'singleGossip' | 'root' | 'max';
// Processed（已处理）:
// 这是最基本的确认级别。当一个交易被验证节点接收并处理，但尚未被确认为有效时，它处于 "Processed" 状态。
// 这个级别的确认表示交易已经到达网络并被一个或多个节点看到，但还没有足够的信息来保证它会被记录在区块链上。

// Confirmed（已确认）:
// 当交易被超过2/3的验证节点验证并认为是有效的时，它达到了 "Confirmed" 状态。
// 这意味着交易已经被网络的大多数节点接受，并且很可能最终会被记录在区块链上。然而，理论上在极端情况下，这个状态的交易仍然有可能被回滚

// Finalized（已最终确定）:
// 最高级别的确认是 "Finalized"，在 Solana 中也被称为 "Rooted"。当交易不仅被验证节点接受，并且被确定为将被永久记录在区块链上时，它就达到了这个状态。
// 一旦交易被定为 "Finalized"，它就被认为是最终确定的，无法回滚。这意味着交易已经完全安全，可以被完全信赖。
