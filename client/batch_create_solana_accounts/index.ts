// 导入所需的包和函数
import { getKeypairFromEnvironment } from '@solana-developers/node-helpers'
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js'
import 'dotenv/config' // 使用 dotenv 来加载环境变量，便于管理敏感信息

// 连接到 Solana 的 devnet，devnet 是 Solana 的开发网络，用于开发和测试，confirmed 是交易确认级别
const connection = new Connection('https://api.devnet.solana.com', 'confirmed')

// 从环境变量中获取发起人账户的密钥对
const secretKey = getKeypairFromEnvironment('SECRET_KEY').secretKey
const payer = Keypair.fromSecretKey(secretKey)

// 准备为新账户分配的 SOL 数量，这里以 lamports 为单位（1 SOL = 1,000,000,000 lamports）
// LAMPORTS_PER_SOL 是 Solana 提供的一个常量，用于表示 1 SOL 对应的 lamports 数量
const lamportsPerAccount = LAMPORTS_PER_SOL * 0.001 // 为每个新账户分配 0.001 SOL

// 指定要创建的账户数量
const numberOfAccounts = process.argv[2] ? parseInt(process.argv[2], 10) : 5 // 可以根据需要调整这个数量

// 定义一个异步函数来批量创建账户
async function createMultipleAccounts() {
  for (let i = 0; i < numberOfAccounts; i++) {
    // 为每个新账户生成一个密钥对
    const newAccount = Keypair.generate()

    // 构建一个交易，用于创建新账户
    // SystemProgram.createAccount 是一个指令，用于在 Solana 上创建新的账户
    // Transaction 对象代表一次 Solana 网络上的交易，可以包含一个或多个指令
    const transaction = new Transaction().add(
      // SystemProgram.createAccount 是一个系统程序，用于创建新的 Solana 账户
      SystemProgram.createAccount({
        fromPubkey: payer.publicKey, // 发起人（支付 SOL 的账户）的公钥
        newAccountPubkey: newAccount.publicKey, // 新账户的公钥
        lamports: lamportsPerAccount, // 分配给新账户的 lamports 数量
        space: 0, // 新账户需要的存储空间大小，这里设置为 0，因为我们不在账户中存储任何数据
        programId: SystemProgram.programId, // 指定账户将由系统程序管理
      }),
    )

    try {
      // 发送交易并等待确认
      const signature = await sendAndConfirmTransaction(
        connection,
        transaction,
        [payer, newAccount], // 此交易需要由发起人和新账户的密钥对签名
        {
          commitment: 'confirmed', // 指定交易确认的级别
          preflightCommitment: 'confirmed',
        },
      )
      console.log(
        `账户 ${i + 1} 创建成功: ${newAccount.publicKey.toBase58()}，分配了 ${lamportsPerAccount / LAMPORTS_PER_SOL} SOL`,
      )
      console.log(`交易签名: ${signature}`)
    } catch (error) {
      console.error(`创建账户 ${i + 1} 失败:`, error)
    }
  }
}

// 调用函数开始批量创建账户
createMultipleAccounts()

// demo output:
// Account created: C22hJSqf3SPPNTMKzasjdAaUd6SwViK1n4fMTj8f1TGh with 0.01 SOL
// Transaction signature: 5GHjw78vFtXDZUk532fxsmYc2JXow7osJQGWrfvEiABGhkbyichBZPo19iEuYxNc6w1i3qgmxWgyZyJ7T8Dj18KY
