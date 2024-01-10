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
