import * as token from '@solana/spl-token'
import * as web3 from '@solana/web3.js'

/**
 * 创建一个用于在 Solana 区块链上撤销代理权限的交易。
 *
 * 代理一旦被撤销，就无法再从代币账户所有者的账户中转移代币。
 * 从先前授权的数量中任何未转移的剩余部分也将无法被代理转移。
 *
 * @param account - 要撤销代理权限的代币账户的公钥。
 * @param owner - 代币账户所有者的公钥。
 *
 * @returns 返回一个 `web3.Transaction` 对象，可以被用来发送交易到 Solana 网络以完成代理的撤销。
 *          这个交易需要由代币账户的所有者或其授权人签名。
 *
 * @usage
 *  - 适用于在应用程序或智能合约中实现代理权限的撤销操作。
 */
async function buildRevokeTransaction(
  account: web3.PublicKey,
  owner: web3.PublicKey,
): Promise<web3.Transaction> {
  // 创建一个新的交易并加入撤销代理权限的指令
  const transaction = new web3.Transaction().add(
    token.createRevokeInstruction(
      account, // 代币账户
      owner, // 账户的所有者
    ),
  )

  // 返回构建的交易
  return transaction
}

export default buildRevokeTransaction
