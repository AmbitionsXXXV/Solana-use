import { getKeypairFromEnvironment } from '@solana-developers/node-helpers'
import 'dotenv/config'

/**
 * 从环境变量中获取 sol SECRET_KEY
 */
const keypair = getKeypairFromEnvironment('SECRET_KEY')

console.log(
  `✅ Finished! We've loaded our secret key securely, using an env file!`,
  keypair,
)
