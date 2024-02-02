import { Keypair } from '@solana/web3.js'
import 'dotenv/config'
import { promises as fs } from 'fs'
import path from 'path'

// 定义批量生成账户的数量
const NUM_ACCOUNTS = 10 // 你可以根据需要调整这个值

/**
 * 批量生成账户并保存到 accounts.json 文件
 * @param numAccounts 批量生成账户的数量
 */
const generateAndSaveAccounts = async (numAccounts: number) => {
  const accounts = []

  for (let i = 0; i < numAccounts; i++) {
    const keypair = Keypair.generate()
    accounts.push({
      publicKey: keypair.publicKey.toBase58(),
      secretKey: Array.from(keypair.secretKey), // 将Uint8Array转换为数组，以便于JSON序列化
    })
  }

  try {
    // 使用fs.promises API异步写文件
    await fs.writeFile(
      path.resolve(__dirname, './accounts.json'),
      JSON.stringify(accounts, null, 2),
    )
    console.log(
      `✅ Successfully generated and saved ${numAccounts} accounts to accounts.json`,
    )
  } catch (err) {
    console.error('Failed to save accounts:', err)
  }
}

// 调用异步函数
await generateAndSaveAccounts(NUM_ACCOUNTS)
