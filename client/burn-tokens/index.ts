import * as token from "@solana/spl-token"
import * as web3 from "@solana/web3.js"

/**
 * 创建一个用于在 Solana 区块链上销毁代币的交易。
 *
 * 销毁代币的过程会减少特定代币发行的供应量。销毁操作将代币从特定代币账户中移除，
 * 并从更广泛的流通中移除这些代币。
 *
 * @param account - 要从中销毁代币的代币账户的公钥。
 * @param mint - 与代币账户关联的代币发行的公钥。
 * @param owner - 代币账户所有者的公钥。
 * @param amount - 要销毁的代币数量。
 *
 * @returns 返回一个 `web3.Transaction` 对象，可以被用来发送交易到 Solana 网络以完成代币销毁。
 *          这个交易需要由代币账户的所有者或其授权人签名。
 *
 * @usage
 *  - 适用于在应用程序或智能合约中实现代币的销毁操作。
 */
async function buildBurnTransaction(
	account: web3.PublicKey,
	mint: web3.PublicKey,
	owner: web3.PublicKey,
	amount: number,
): Promise<web3.Transaction> {
	// 创建一个新的交易并加入代币销毁的指令
	const transaction = new web3.Transaction().add(
		token.createBurnInstruction(
			account, // 代币账户
			mint, // 代币发行
			owner, // 账户的所有者
			amount, // 销毁数量
		),
	)

	// 返回构建的交易
	return transaction
}

export default buildBurnTransaction
