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

// 此函数用于构建铸造代币的交易
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
