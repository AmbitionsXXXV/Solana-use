import { getKeypairFromEnvironment } from "@solana-developers/helpers"
import {
	Connection,
	Keypair,
	SystemProgram,
	Transaction,
	clusterApiUrl,
	sendAndConfirmTransaction,
} from "@solana/web3.js"
import bs58 from "bs58"
import "dotenv/config"
import fs from "node:fs"
import path from "node:path"
import process from "node:process"

// 使用 npx index.ts <文件路径> <amount> 运行

const directoryPath = process.argv[2] || process.cwd() // 如果没有提供文件路径，则使用当前工作目录
const amount = Number(process.argv[3]) || 1

const fromKeypair = getKeypairFromEnvironment("SECRET_KEY")

// 将提供的接收者公钥字符串转换为 PublicKey 对象
const fromPubkey = fromKeypair.publicKey

// 创建到 Solana devnet 的连接
const connection = new Connection(clusterApiUrl("mainnet-beta"), "confirmed")

// 确认已加载发送者密钥对，接收者公钥，并且已连接到 Solana 网络
console.log(
	"✅ Loaded our own keypair, the destination public key, and connected to Solana",
)

const keypairs: Keypair[] = []

const files = fs.readdirSync(directoryPath)

for (const file of files) {
	if (path.extname(file) === ".json") {
		const secretKey = JSON.parse(
			fs.readFileSync(path.join(directoryPath, file), "utf-8"),
		)
		const decodedSecretKey = Array.isArray(secretKey)
			? Uint8Array.from(secretKey)
			: bs58.decode(secretKey)

		keypairs.push(Keypair.fromSecretKey(decodedSecretKey))
	}
}

// 确认已加载密钥对，并且已连接到 Solana 网络
console.log("✅ Loaded keypairs and connected to Solana")

// 获取最新的区块哈希
const { blockhash } = await connection.getLatestBlockhash()

// 创建一个新的交易对象
const transaction = new Transaction({
	recentBlockhash: blockhash,
	feePayer: fromKeypair.publicKey, // 设置手续费支付者
})
let count = 0

// 循环遍历每个地址并发送转账
for (const keypair of keypairs) {
	try {
		if (count === 20) break

		const balance = await connection.getBalance(fromPubkey)

		if (balance < amount) {
			console.log(`账户余额不足：${balance / 1000000000} Sol`)
			break
		}

		// 定义要发送的 lamports 数量（1 SOL = 1,000,000,000 lamports）
		const LAMPORTS_TO_SEND = amount * 10 ** 9 // 保留一些余额以支付手续费

		console.log("from keypair:", keypair.publicKey.toBase58())
		console.log("fromPubkey:", fromPubkey.toBase58())

		// 创建转账指令
		const sendSolInstruction = SystemProgram.transfer({
			fromPubkey,
			toPubkey: keypair.publicKey,
			lamports: LAMPORTS_TO_SEND,
		})

		// 将转账指令添加到交易中
		transaction.add(sendSolInstruction)

		// 打印转账成功的消息和交易签名
		console.log(
			`💸 Send ${LAMPORTS_TO_SEND / 1000000000} sol from ${fromPubkey} to ${keypair.publicKey}.`,
		)

		console.log(
			">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>",
		)
	} catch (error) {
		console.error(
			`⚠️ Error processing keypair ${keypair.publicKey.toBase58()}:`,
			error,
		)
	}

	count++
}

// 发送交易并等待确认
const signature = await sendAndConfirmTransaction(connection, transaction, [
	fromKeypair,
])

console.log(
	`Transaction signature: https://explorer.solana.com/tx/${signature}`,
)

// 打印最终余额
console.log(
	`Final balance: ${(await connection.getBalance(fromPubkey)) / 1000000000} Sol`,
)
