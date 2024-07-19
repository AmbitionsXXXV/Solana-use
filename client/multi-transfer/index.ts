import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  clusterApiUrl,
  sendAndConfirmTransaction,
} from '@solana/web3.js'
import bs58 from 'bs58'
import fs from 'node:fs'
import path from 'node:path'
import process from 'node:process'

// 使用 npx index.ts <归集地址> <文件路径> 运行

// 从命令行参数中获取接收者的公钥和文件路径
const suppliedToPubkey = process.argv[2] || null
const directoryPath = process.argv[3] || process.cwd() // 如果没有提供文件路径，则使用当前工作目录

// 如果没有提供接收者公钥或文件路径，则提示用户输入并退出程序
if (!suppliedToPubkey) {
  console.log('填写归集地址')
  process.exit(1)
}

// 将提供的接收者公钥字符串转换为 PublicKey 对象
const toPubkey = new PublicKey(suppliedToPubkey)

// 创建到 Solana devnet 的连接
const connection = new Connection(clusterApiUrl('mainnet-beta'), 'confirmed')

// 确认已加载发送者密钥对，接收者公钥，并且已连接到 Solana 网络
console.log(
  `✅ Loaded our own keypair, the destination public key, and connected to Solana`,
)

let keypairs: Keypair[] = []

const files = fs.readdirSync(directoryPath)

for (const file of files) {
  if (path.extname(file) === '.json') {
    const secretKey = JSON.parse(
      fs.readFileSync(path.join(directoryPath, file), 'utf-8'),
    )
    const decodedSecretKey = Array.isArray(secretKey)
      ? Uint8Array.from(secretKey)
      : bs58.decode(secretKey)

    keypairs.push(Keypair.fromSecretKey(decodedSecretKey))
  }
}

// 确认已加载密钥对，并且已连接到 Solana 网络
console.log(`✅ Loaded keypairs and connected to Solana`)

// 循环遍历每个地址并发送转账
for (const keypair of keypairs) {
  try {
    const balance = await connection.getBalance(keypair.publicKey)
    const minimum = await connection.getMinimumBalanceForRentExemption(0)

    console.log(`账户最小需要的租金为：${minimum / 1000000000} Sol`)

    // 创建一个新的交易对象
    const transaction = new Transaction()

    // 定义要发送的 lamports 数量（1 SOL = 1,000,000,000 lamports）
    const LAMPORTS_TO_SEND = balance - minimum - 5500 // 保留一些余额以支付手续费

    console.log('from keypair:', keypair.publicKey.toBase58())
    console.log('toPubkey:', toPubkey.toBase58())

    // 创建转账指令
    const sendSolInstruction = SystemProgram.transfer({
      fromPubkey: keypair.publicKey,
      toPubkey,
      lamports: LAMPORTS_TO_SEND,
    })

    // 将转账指令添加到交易中
    transaction.add(sendSolInstruction)

    // 发送交易并等待确认
    const signature = await sendAndConfirmTransaction(connection, transaction, [
      keypair,
    ])

    // 打印转账成功的消息和交易签名
    console.log(
      `💸 Sent ${LAMPORTS_TO_SEND / 1000000000} sol from ${keypair.publicKey} to ${toPubkey}.`,
    )
    console.log(`Transaction signature: https://explorer.solana.com/tx/${signature}`)
    console.log('>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>')
  } catch (error) {
    console.error(
      `⚠️ Error processing keypair ${keypair.publicKey.toBase58()}:`,
      error,
    )
  }
}

// 打印最终余额
console.log(
  `Final balance: ${(await connection.getBalance(toPubkey)) / 1000000000} Sol`,
)
