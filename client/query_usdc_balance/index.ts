// import { TOKEN_PROGRAM_ID } from '@solana/spl-token' // 导入 Solana 代币程序的 ID，用于与代币相关的操作
import {
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  clusterApiUrl,
} from '@solana/web3.js'
import 'dotenv/config'

/**
 * 运行：npx esrun client/query_usdc_balance/index.ts ,提供了默认地址
 * 如果需要指定用户地址，可以在命令行中传入，例如：npx esrun client/query_usdc_balance/index.ts <需要查询的 Solana 地址>
 */

// 设置连接到 Solana 网络的集群名称，这里使用 'devnet' 作为测试网络
const CLUSTER_NAME = 'devnet'
// 想查看主网的话，可以使用 'mainnet-beta'，然后下方 userWalletAddress 的 new PublicKey 使用主网地址

// 使用 clusterApiUrl 创建到指定 Solana 集群（网络）的连接
const connection = new Connection(clusterApiUrl(CLUSTER_NAME))

// 通过环境变量获取 USDC 代币在 Solana devnet 上的地址
const usdcTokenMintAddress = new PublicKey(process.env.USDC_DEVNET)
console.log('USDC Token Mint Address:', usdcTokenMintAddress.toBase58())

// 用户的 Solana 地址，这里使用一个示例地址
const userWalletAddress = new PublicKey(
  process.argv[2] || '9zofpcQiKYW5f3M2NtSZyxM89mzPhNwiZv9FCXtFvVuE',
)

// 如果用户账户余额为 0，则向用户账户请求 1 个 SOL
if ((await connection.getBalance(userWalletAddress)) === 0) {
  await connection.requestAirdrop(userWalletAddress, LAMPORTS_PER_SOL * 1)
}

// 定义一个异步函数来查询账户的 USDC 余额
async function findUsdcBalance() {
  // 获取与特定 USDC 代币相关联的用户代币账户信息
  const userTokenAccountInfo = await connection.getParsedTokenAccountsByOwner(
    userWalletAddress,
    { mint: usdcTokenMintAddress },
  )

  // 检查是否找到了代币账户，并打印余额
  if (userTokenAccountInfo.value.length > 0) {
    // 解析并打印 USDC 余额
    const usdcBalance =
      userTokenAccountInfo.value[0].account.data.parsed.info.tokenAmount.uiAmount
    console.log(`USDC Balance: ${usdcBalance}`)
  } else {
    console.log('No USDC account found.')
  }

  // 获取并打印用户所有代币账户的余额信息，如果需要可以取消注释
  // const balances = await connection.getParsedTokenAccountsByOwner(
  //   userWalletAddress,
  //   {
  //     programId: TOKEN_PROGRAM_ID, // 指定 Solana 代币程序 ID 来筛选所有代币账户
  //   },
  // )

  // console.log('Balances:', balances)
}

// 调用函数，开始查询 USDC 余额
findUsdcBalance()
