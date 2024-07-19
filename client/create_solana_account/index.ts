import { Keypair } from '@solana/web3.js'; // 导入Keypair类，用于生成Solana账户的公钥和私钥
import 'dotenv/config'; // 导入dotenv/config模块，用于加载环境变量
import { promises as fs } from 'node:fs'; // 导入fs模块的promises API，用于异步文件操作
import path from 'node:path'; // 导入path模块，用于处理文件和目录的路径

// 定义批量生成账户的数量
const NUM_ACCOUNTS = 80 // 你可以根据需要调整这个值

/**
 * 批量生成 Solana 基本账户并保存到 accounts.json 文件
 * @param numAccounts 批量生成账户的数量
 */
const generateAndSaveAccounts = async (numAccounts: number) => {
  const accounts = [] // 创建一个空数组，用于存储生成的账户信息
  const scripts = []

  for (let i = 0; i < numAccounts; i++) {
    // 使用 Keypair 的 generate 方法生成一个新的密钥对，这是一个 Solana 账户的基础
    const keypair = Keypair.generate()

    accounts.push({
      publicKey: keypair.publicKey.toBase58(), // 公钥用于接收资金，使用 toBase58 方法转换为 Base58 格式，这是 Solana 公钥的常见表示方式
      secretKey: Array.from(keypair.secretKey), // 私钥用于签名交易，是一个 Uint8Array，使用 Array.from 方法转换为数组，以便于 JSON 序列化
    })

    const keyFileName = `key${i}.json`
    const scriptFileName = `mine_key${i}.sh`
    const scriptClaimFileName = `claim_key${i}.sh`

    scripts.push({
      name: `miner-${i}`,
      script: `./${scriptFileName}`,
      interpreter: '/bin/bash',
    })
    scripts.push({
      name: `claim-${i}`,
      script: `./${scriptClaimFileName}`,
      interpreter: '/bin/bash',
      restart_delay: 1000 * 60 * 30,
    })

    // 保存密钥文件
    await fs.writeFile(
      path.resolve(__dirname, keyFileName),
      JSON.stringify(Array.from(keypair.secretKey)),
    )

    const scriptClaimContent = `#!/bin/bash
    ore \\
      --rpc https://fluent-crimson-asphalt.solana-mainnet.quiknode.pro/536344f59d2175300396d8a601ccb2841f43ac8b/ \
      --keypair ~/.config/solana//${keyFileName} \\
      --priority-fee 300 \\
      claim
    `

    // 创建并保存脚本文件
    const scriptContent = `#!/bin/bash
ore \\
  --rpc https://fluent-crimson-asphalt.solana-mainnet.quiknode.pro/536344f59d2175300396d8a601ccb2841f43ac8b/ \\
  --keypair ~/.config/solana/${keyFileName} \\
  --priority-fee 5000 \\
  mine \\
  --threads 24
`
    await fs.writeFile(path.resolve(__dirname, scriptFileName), scriptContent)
    await fs.writeFile(
      path.resolve(__dirname, scriptClaimFileName),
      scriptClaimContent,
    )
  }

  try {
    await fs.writeFile(
      path.resolve(__dirname, 'ecosystem.config.js'),
      `module.exports = {
        apps: ${JSON.stringify(scripts, null, 2)}
      };`,
    )
    console.log(
      `✅ Successfully generated and saved ${numAccounts} accounts to accounts.json`,
    )
  } catch (err) {
    console.error('Failed to save accounts:', err)
  }
}

await generateAndSaveAccounts(NUM_ACCOUNTS)

// 调用异步函数，开始生成账户并保存到文件
await generateAndSaveAccounts(NUM_ACCOUNTS)
