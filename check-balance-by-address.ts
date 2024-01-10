import { Connection, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js'

/**
 * 通过运行时参数获取公钥，并检查余额
 * 使用：npx esrun check-balance-by-address.ts (some wallet address)
 */
const suppliedPublicKey = process.argv[2]
if (!suppliedPublicKey) {
  throw new Error('Provide a public key to check the balance of!')
}

const connection = new Connection('https://api.devnet.solana.com', 'confirmed')

const publicKey = new PublicKey(suppliedPublicKey)

const balanceInLamports = await connection.getBalance(publicKey)

const balanceInSOL = balanceInLamports / LAMPORTS_PER_SOL

console.log(
  `✅ Finished! The balance for the wallet at address ${publicKey} is ${balanceInSOL}!`,
)
