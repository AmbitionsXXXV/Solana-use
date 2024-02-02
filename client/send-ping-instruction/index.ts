import { getKeypairFromEnvironment } from '@solana-developers/node-helpers'
import web3 from '@solana/web3.js'
import 'dotenv/config'

/**
 * Solana Ping Transaction Script
 *
 * 这个脚本用于在 Solana 区块链上执行一个特定的 "PING" 操作。主要步骤包括：
 * 1. 环境设置：从环境变量中获取支付者的密钥对，以及导入必要的依赖和配置。
 * 2. 连接到网络：建立与 Solana Devnet 网络的连接。
 * 3. 定义 PING 交易：设置 PING 程序的地址和与该程序相关的数据地址。
 * 4. 创建和发送交易：定义一个异步函数 sendPingTransaction 来创建和发送交易。
 *    这个交易包含向特定程序发送数据的指令。
 * 5. 交易确认和输出：交易完成后，输出交易的签名，并提供在 Solana Explorer 上查看交易的链接。
 * 6. 空投 SOL（可选）：如果钱包中的 SOL 不足以支付交易费用，提供了一个代码段用于空投 SOL。
 *
 * 这个脚本展示了如何在 Solana 区块链上发送交易和与智能合约交互的基本步骤。
 */
// PING 程序的公钥地址
const PING_PROGRAM_ADDRESS = new web3.PublicKey(
  'ChT1B39WKLS8qUrkLvFDXMhEJ4F1XZzwUNHUt4AU9aVa',
)
// PING 程序数据的公钥地址
const PING_PROGRAM_DATA_ADDRESS = new web3.PublicKey(
  'Ah9K7dQ8EHaZqcAsgBW8w37yN2eAy3koFmUn4x3CJtod',
)

const CLUSTER_NAME = 'devnet'

// 从环境变量获取支付者的密钥对
const payer = getKeypairFromEnvironment('SECRET_KEY')
console.log(`🔑 Loaded keypair ${payer.publicKey.toBase58()}!`)

// 创建到 Solana Devnet 的连接
const connection = new web3.Connection(web3.clusterApiUrl(CLUSTER_NAME))
console.log(`⚡️ Connected to Solana ${CLUSTER_NAME} cluster!`)

// 发送 PING 交易的异步函数
async function sendPingTransaction(
  connection: web3.Connection,
  payer: web3.Keypair,
) {
  // 创建新的交易对象
  const transaction = new web3.Transaction()

  // 转换为 PublicKey 类型
  const programId = new web3.PublicKey(PING_PROGRAM_ADDRESS)
  const pingProgramDataId = new web3.PublicKey(PING_PROGRAM_DATA_ADDRESS)

  // 创建交易指令
  // TransactionInstruction 这种方式提供了更高的灵活性，允许开发者直接定义交易的细节。它常用于与自定义程序（智能合约）交互，特别是当需要提供特定的参数或与非标准程序交互时
  // 直接创建 TransactionInstruction：适用于需要更细粒度控制的场景，比如与自定义智能合约交互。这种方法提供了更大的灵活性，允许开发者明确指定交易中的每个参与账户的角色和权限
  const instruction = new web3.TransactionInstruction({
    keys: [
      {
        pubkey: pingProgramDataId,
        // isSigner 表示账户是否是交易的签字人
        isSigner: false,
        // isWritable 表示在交易执行过程中是否写入账户信息
        isWritable: true,
      },
    ],
    programId,
  })

  // 将指令添加到交易中
  transaction.add(instruction)

  // 发送并确认交易
  const signature = await web3.sendAndConfirmTransaction(connection, transaction, [
    payer,
  ])

  // 输出交易完成信息和签名
  console.log(`✅ Transaction completed! Signature is ${signature}`)

  // 提供在 Solana Explorer 中查看交易的链接
  console.log(
    `You can view your transaction on the Solana Explorer at:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`,
  )
}

// 如果你的钱包没有足够的 SOL，你可以使用下面的代码向你的钱包发送 1 SOL：
if ((await connection.getBalance(payer.publicKey)) === 0) {
  await connection.requestAirdrop(payer.publicKey, web3.LAMPORTS_PER_SOL * 1)
}

// 调用发送 PING 交易的函数
await sendPingTransaction(connection, payer)
