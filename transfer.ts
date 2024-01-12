import { getKeypairFromEnvironment } from '@solana-developers/node-helpers'
import {
  Connection,
  PublicKey,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js'
import 'dotenv/config'

/**
 *
 */
const suppliedToPubkey = process.argv[2] || null

if (!suppliedToPubkey) {
  console.log(`Please provide a public key to send to`)
  process.exit(1)
}

const senderKeypair = getKeypairFromEnvironment('SECRET_KEY')

console.log(`suppliedToPubkey: ${suppliedToPubkey}`, senderKeypair)

const toPubkey = new PublicKey(suppliedToPubkey)

const connection = new Connection('https://api.devnet.solana.com', 'confirmed')

console.log(
  `✅ Loaded our own keypair, the destination public key, and connected to Solana`,
)

const transaction = new Transaction()

const LAMPORTS_TO_SEND = 5000

/**
 * The `SystemProgram.transfer()` function requires:
 * a public key corresponding to the sender account
 * a public key corresponding to the recipient account
 * the amount of SOL to send in lamports.
 */
const sendSolInstruction = SystemProgram.transfer({
  fromPubkey: senderKeypair.publicKey,
  toPubkey,
  lamports: LAMPORTS_TO_SEND,
})

transaction.add(sendSolInstruction)

const signature = await sendAndConfirmTransaction(connection, transaction, [
  senderKeypair,
])

console.log(`💸 Finished! Sent ${LAMPORTS_TO_SEND} to the address ${toPubkey}. `)
console.log(`Transaction signature is ${signature}!`)

// How much SOL did the transfer take? What is this in USD?
// 转移的 SOL 数量是 5000 / 1,000,000,000 SOL。

// Can you find your transaction on https://explorer.solana.com? Remember we are using the devnet network.

// How long does the transfer take?

// What do you think "confirmed" means?
// type Commitment = 'processed' | 'confirmed' | 'finalized' | 'recent' | 'single' | 'singleGossip' | 'root' | 'max';
// Processed（已处理）:
// 这是最基本的确认级别。当一个交易被验证节点接收并处理，但尚未被确认为有效时，它处于 "Processed" 状态。
// 这个级别的确认表示交易已经到达网络并被一个或多个节点看到，但还没有足够的信息来保证它会被记录在区块链上。

// Confirmed（已确认）:
// 当交易被超过2/3的验证节点验证并认为是有效的时，它达到了 "Confirmed" 状态。
// 这意味着交易已经被网络的大多数节点接受，并且很可能最终会被记录在区块链上。然而，理论上在极端情况下，这个状态的交易仍然有可能被回滚

// Finalized（已最终确定）:
// 最高级别的确认是 "Finalized"，在 Solana 中也被称为 "Rooted"。当交易不仅被验证节点接受，并且被确定为将被永久记录在区块链上时，它就达到了这个状态。
// 一旦交易被定为 "Finalized"，它就被认为是最终确定的，无法回滚。这意味着交易已经完全安全，可以被完全信赖。
