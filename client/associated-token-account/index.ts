import * as token from '@solana/spl-token'
import * as web3 from '@solana/web3.js'

// 这个函数用于构建创建关联代币账户的交易
async function buildCreateAssociatedTokenAccountTransaction(
  payer: web3.PublicKey, // 付款人的公钥
  mint: web3.PublicKey, // 代币发行的公钥
): Promise<web3.Transaction> {
  // 获取关联代币账户的地址
  const associatedTokenAddress = await token.getAssociatedTokenAddress(
    mint,
    payer,
    false,
  )

  // 创建一个新的交易并加入创建关联代币账户的指令
  const transaction = new web3.Transaction().add(
    token.createAssociatedTokenAccountInstruction(
      payer, // 付款人
      associatedTokenAddress, // 关联代币账户地址
      payer, // 代币账户的所有者（在此例中，所有者与付款人相同）
      mint, // 代币发行
    ),
  )

  // 返回构建的交易
  return transaction
}

export default buildCreateAssociatedTokenAccountTransaction
