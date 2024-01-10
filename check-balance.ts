import {
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  clusterApiUrl,
} from '@solana/web3.js'

/**
 * 检查连接，并获取账户余额
 * 1,000,000,000 lamports = 1 sol
 */
const connection = new Connection(clusterApiUrl('devnet'))
const address = new PublicKey('9zofpcQiKYW5f3M2NtSZyxM89mzPhNwiZv9FCXtFvVuE')
const balance = await connection.getBalance(address)
// 计算余额 sol
const balanceInSol = balance / LAMPORTS_PER_SOL

console.log(`The balance of the account at ${address} is ${balanceInSol} SOL`)
console.log(`✅ Finished!`)
