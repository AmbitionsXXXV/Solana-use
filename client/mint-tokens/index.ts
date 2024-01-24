import * as token from '@solana/spl-token'
import * as web3 from '@solana/web3.js'

// const transactionSignature = await token.mintTo(
//   connection,
//   payer,
//   mint,
//   destination,
//   authority,
//   amount
// )

/**
 * 创建一个用于在 Solana 区块链上铸造代币的交易。
 *
 * 这个函数封装了构建铸币交易的过程，使得与 Solana 区块链的交互更为简化。
 *
 * @param authority - 拥有铸造该代币权限的账户的公钥。
 *                    这个账户必须有权限向指定的代币账户铸造代币。
 * @param mint - 代币的发行地址，即代币的唯一标识符。
 * @param amount - 要铸造的代币数量，这个数值不考虑代币的小数位。
 * @param destination - 将接收新铸造代币的账户地址。
 *
 * @returns 返回一个 `web3.Transaction` 对象，可以被用来发送交易到 Solana 网络以完成铸币过程。
 *          这个交易需要由拥有铸币权限的账户签名。
 *
 * @usage
 *  - 适用于为用户铸造新代币，例如在代币的初始发行、奖励分发或其他类似场景。
 *  - 适用于去中心化应用（DApp）中实现代币的动态生成或发行。
 */
async function buildMintToTransaction(
  authority: web3.PublicKey, // 授权铸造代币的账户的公钥
  mint: web3.PublicKey, // 代币发行的公钥
  amount: number, // 要铸造的代币数量（不考虑小数点）
  destination: web3.PublicKey, // 代币将被铸造到的目标账户的公钥
): Promise<web3.Transaction> {
  // 创建一个新的交易并加入铸造代币的指令
  const transaction = new web3.Transaction().add(
    token.createMintToInstruction(
      mint, // 代币发行
      destination, // 目标账户
      authority, // 授权账户
      amount, // 铸造数量
    ),
  )

  // 返回构建的交易
  return transaction
}

export default buildMintToTransaction
