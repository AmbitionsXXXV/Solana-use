import { Keypair } from '@solana/web3.js'

/**
 * 生成 sol 公私钥对
 */
const keypair = Keypair.generate()

console.log(`The public key is: `, keypair.publicKey.toBase58())
console.log(`The secret key is: `, keypair.secretKey)
