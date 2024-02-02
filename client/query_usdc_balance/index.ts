import { Connection, PublicKey } from '@solana/web3.js'
import 'dotenv/config'

// 设置网络连接
const connection = new Connection('https://api.mainnet-beta.solana.com')

// USDC 代币在 Solana 主网上的地址
const usdcTokenMintAddress = new PublicKey(process.env.USDC_DEVNET)

console.log('USDC Token Mint Address:', usdcTokenMintAddress.toBase58())

// 用户的 Solana 地址
const userWalletAddress = new PublicKey(
  '9zofpcQiKYW5f3M2NtSZyxM89mzPhNwiZv9FCXtFvVuE',
)

// 查询账户的 USDC 余额
async function findUsdcBalance() {
  // 获取 USDC 代币的账户信息
  const userTokenAccountInfo = await connection.getParsedTokenAccountsByOwner(
    userWalletAddress,
    { mint: usdcTokenMintAddress },
  )

  if (userTokenAccountInfo.value.length > 0) {
    const usdcBalance =
      userTokenAccountInfo.value[0].account.data.parsed.info.tokenAmount.uiAmount
    console.log(`USDC Balance: ${usdcBalance}`)
  } else {
    console.log('No USDC account found.')
  }
}

findUsdcBalance()
