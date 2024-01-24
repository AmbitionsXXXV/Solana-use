import * as token from '@solana/spl-token'
import * as web3 from '@solana/web3.js'

/**
 * 创建一个用于在 Solana 区块链上转移代币的交易。
 *
 * 在转移代币之前，发送方和接收方都必须拥有与代币发行相对应的代币账户。
 * 可以使用 getOrCreateAssociatedTokenAccount 来获取接收方的关联代币账户，
 * 以确保在转账之前他们的代币账户已存在。如果账户不存在，这个函数将创建账户，
 * 并从交易的付款人账户中扣除创建账户所需的 lamports。
 *
 * @param source - 发送代币的源代币账户的公钥。
 * @param destination - 接收代币的目标代币账户的公钥。
 * @param owner - 源代币账户所有者的公钥。
 * @param amount - 要转移的代币数量。
 *
 * @returns 返回一个 `web3.Transaction` 对象，可以被用来发送交易到 Solana 网络以完成代币转移。
 *          这个交易需要由源代币账户的所有者或其授权人签名。
 *
 * @usage
 *  - 适用于在应用程序或智能合约中实现代币的转移操作。
 */
async function buildTransferTransaction(
  source: web3.PublicKey,
  destination: web3.PublicKey,
  owner: web3.PublicKey,
  amount: number,
): Promise<web3.Transaction> {
  // 创建一个新的交易并加入代币转移的指令
  const transaction = new web3.Transaction().add(
    token.createTransferInstruction(
      source, // 源代币账户
      destination, // 目标代币账户
      owner, // 源账户的所有者
      amount, // 转移数量
    ),
  )

  // 返回构建的交易
  return transaction
}

export default buildTransferTransaction
