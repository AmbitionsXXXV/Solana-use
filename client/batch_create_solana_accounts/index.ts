import { getKeypairFromEnvironment } from '@solana-developers/helpers'
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js'
import 'dotenv/config' // 使用 dotenv 来加载环境变量，便于管理敏感信息
import { promises as fs } from 'fs' // 导入fs模块的promises API，用于异步文件操作
import path from 'path'

/**
 * 运行前准备：
 * ❗❗❗保证发起人账户有足够的 SOL 用于支付交易费用，因为创建账户需要支付交易费用
 *
 * 运行
 * npx esrun client/batch_create_solana_accounts/index.ts
 * 如果你想要创建不同数量的账户，可以在命令行中传入一个参数，例如：
 * npx esrun client/batch_create_solana_accounts/index.ts 10 // 创建10个账户
 */

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
  let accountDetails = [] // 用于存储所有新创建账户的详情

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
        // 关于 programId：在 Solana 上，账户的行为由程序（Program）控制，每个账户都有一个关联的程序，这个程序的公钨称为 programId
        // 如果想要自己编写程序来控制账户的行为，可以使用自定义的 programId，自定义程序的开发和部署需要使用 Rust 编程语言和 Solana 提供的开发工具
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

      // 保存账户的公钥和私钥到数组
      accountDetails.push({
        publicKey: newAccount.publicKey.toBase58(),
        secretKey: Array.from(newAccount.secretKey), // 将 Uint8Array 转换为普通数组
      })
    } catch (error) {
      console.error(`创建账户 ${i + 1} 失败:`, error)
    }
  }

  // 将所有账户信息写入 JSON 文件
  try {
    // 使用 fs.promises API 的 writeFile 方法异步写文件
    await fs.writeFile(
      path.resolve(__dirname, './createdAccounts.json'),
      JSON.stringify(accountDetails, null, 2), // 使用 JSON.stringify 方法将 accounts 数组转换为JSON格式的字符串，JSON.stringify 的第二个和第三个参数用于格式化输出，使得生成的 JSON 文件更易读
    )
    console.log(
      `✅ Successfully generated and saved ${numberOfAccounts} accounts to createdAccounts.json`, // 如果文件写入成功，打印成功信息
    )
  } catch (err) {
    // 如果文件写入失败，打印错误信息
    console.error('Failed to save accounts:', err)
  }
}

// 调用函数开始批量创建账户
createMultipleAccounts()

// demo output:
// Account created: C22hJSqf3SPPNTMKzasjdAaUd6SwViK1n4fMTj8f1TGh with 0.01 SOL
// Transaction signature: 5GHjw78vFtXDZUk532fxsmYc2JXow7osJQGWrfvEiABGhkbyichBZPo19iEuYxNc6w1i3qgmxWgyZyJ7T8Dj18KY
