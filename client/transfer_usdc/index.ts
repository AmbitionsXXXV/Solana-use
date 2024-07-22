import { getKeypairFromEnvironment } from "@solana-developers/helpers"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token"
import {
	Connection,
	Keypair,
	PublicKey,
	type PublicKeyInitData,
	clusterApiUrl,
} from "@solana/web3.js"
import "dotenv/config" // 使用 dotenv 来加载环境变量，方便管理配置和敏感信息
import process from "node:process"

// 从环境变量中读取发送方的私钥，用于创建密钥对
const senderSecretKey = getKeypairFromEnvironment("SECRET_KEY").secretKey
// 从命令行参数或使用默认值获取接收方的公钥地址
const recipientPublicKeyStr =
	process.argv[2] || "3rMgEgiGrsqWbxBiKML14cNwTU3ze8zcvusaqsYzXxJz"
// 通过环境变量获取 USDC 代币的 Mint 地址（即代币的唯一标识符）
const usdcMintAddressStr = new PublicKey(
	process.env.USDC_DEVNET as PublicKeyInitData,
)
// 设置转账金额，注意：最终的金额需要考虑代币的小数位数
const amount = 0.5

/**
 * 运行：npx esrun client/transfer_usdc/index.ts
 * 如果需要指定接收方的公钥，可以在命令行中传入一个参数再运行，例如：
 * npx esrun client/transfer_usdc/index.ts <接收方的公钥>
 */
async function transferUSDC() {
	// 创建到 Solana 网络的连接（这里使用 devnet 测试网络）
	const connection = new Connection(clusterApiUrl("devnet"), "confirmed")
	// 使用发送方的私钥创建密钥对对象
	const senderKeypair = Keypair.fromSecretKey(senderSecretKey)
	// 创建接收方的公钥对象
	const recipientPublicKey = new PublicKey(recipientPublicKeyStr)
	// 创建 USDC 代币 Mint 地址的公钥对象
	const usdcMintAddress = new PublicKey(usdcMintAddressStr)

	// 获取或创建发送方的 USDC 代币账户
	// 如果账户不存在，将自动创建一个与发送方钱包关联的代币账户
	const senderTokenAccount = await getOrCreateAssociatedTokenAccount(
		connection,
		senderKeypair,
		usdcMintAddress,
		senderKeypair.publicKey,
	)

	// 获取或创建接收方的 USDC 代币账户
	// 同样，如果账户不存在，将为接收方创建一个代币账户
	const recipientTokenAccount = await getOrCreateAssociatedTokenAccount(
		connection,
		senderKeypair,
		usdcMintAddress,
		recipientPublicKey,
		true,
	)

	// 执行 USDC 代币的转账操作
	// 这里需要指定转账的金额，注意金额单位是代币的最小单位（例如，USDC 通常有6位小数）
	const signature = await transfer(
		connection,
		senderKeypair,
		senderTokenAccount.address,
		recipientTokenAccount.address,
		senderKeypair.publicKey,
		amount * 10 ** 6, // 转换金额到最小单位
		[],
	)

	// 打印交易签名，用于后续查询和确认交易
	console.log(`转账成功，交易签名：${signature}`)
}

// 执行异步函数，开始转账操作
await transferUSDC()
