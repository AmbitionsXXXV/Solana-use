import { getKeypairFromEnvironment } from '@solana-developers/node-helpers'
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
} from '@solana/web3.js'
import 'dotenv/config'

// 创建新的Keypair
const newAccount = Keypair.generate()

// 连接到Solana的devnet
const connection = new Connection('https://api.devnet.solana.com', 'confirmed')

const secretKey = getKeypairFromEnvironment('SECRET_KEY').secretKey

// 假设你有一个已经准备好的发起人账户
// const payer = Keypair.fromSecretKey(/* 你的发起人账户的私钥 */)
const payer = Keypair.fromSecretKey(secretKey)

// 准备为新账户分配的SOL数量
const lamports = LAMPORTS_PER_SOL * 0.01 // 例如，分配0.01 SOL

async function createAccount() {
  // 构建交易以创建新账户
  const transaction = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: newAccount.publicKey,
      lamports,
      space: 0, // 如果账户需要存储数据，这里应该指定需要的空间大小
      programId: SystemProgram.programId,
    }),
  )

  // 签署并发送交易
  try {
    let signature = await connection.sendTransaction(transaction, [
      payer,
      newAccount,
    ])
    await connection.confirmTransaction(signature, 'confirmed')
    console.log(
      `Account created: ${newAccount.publicKey.toBase58()} with ${lamports / LAMPORTS_PER_SOL} SOL`,
    )
  } catch (error) {
    console.error('Failed to create account:', error)
  }
}

createAccount()
