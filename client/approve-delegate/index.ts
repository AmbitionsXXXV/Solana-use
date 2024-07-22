import * as token from "@solana/spl-token"
import * as web3 from "@solana/web3.js"

/**
 * 创建一个用于在 Solana 区块链上授权代理进行代币操作的交易。
 *
 * 授权代理是指授权另一个账户从特定代币账户转移或销毁代币的过程。
 * 使用代理时，代币账户的控制权仍然在原始所有者手中。
 * 代理可转移或销毁的最大代币数量在代币账户所有者授权代理时指定。
 * 注意，任何时候只能有一个代理账户与代币账户关联。
 *
 * @param account - 要从中委派代币的代币账户的公钥。
 * @param delegate - 所有者授权进行代币转移或销毁的账户的公钥。
 * @param owner - 代币账户所有者的公钥。
 * @param amount - 代理可转移或销毁的最大代币数量。
 *
 * @returns 返回一个 `web3.Transaction` 对象，可以被用来发送交易到 Solana 网络以完成代理授权。
 *          这个交易需要由代币账户的所有者或其授权人签名。
 *
 * @usage
 *  - 适用于在应用程序或智能合约中实现代币的代理授权操作。
 */
async function buildApproveTransaction(
	account: web3.PublicKey,
	delegate: web3.PublicKey,
	owner: web3.PublicKey,
	amount: number,
): Promise<web3.Transaction> {
	// 创建一个新的交易并加入代理授权的指令
	const transaction = new web3.Transaction().add(
		token.createApproveInstruction(
			account, // 代币账户
			delegate, // 代理账户
			owner, // 账户的所有者
			amount, // 授权数量
		),
	)

	// 返回构建的交易
	return transaction
}

export default buildApproveTransaction
