import { getKeypairFromEnvironment } from '@solana-developers/helpers'
import 'dotenv/config'

/**
 * 从环境变量中获取 sol SECRET_KEY
 */
const keypair = getKeypairFromEnvironment('SECRET_KEY')

console.log(
  `✅ Finished! We've loaded our secret key securely, using an env file!`,
  keypair,
)

console.log(`The public key is: `, keypair.publicKey.toBase58())
console.log(`The secret key is: `, keypair.secretKey)
