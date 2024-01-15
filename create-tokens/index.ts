import * as token from '@solana/spl-token'
import * as web3 from '@solana/web3.js'
import { initializeKeypair } from './initializeKeypair'

async function main() {
  // 1. 连接到 Solana 网络。
  // 2. 初始化用户的密钥对。
  // 3. 创建一个新的代币发行（Mint）。
  // 4. 获取新创建的代币发行的信息。
  // 5. 为用户创建一个代币账户。
  // 6. 向代币账户铸造代币。
  // 7. 创建一个接收者的代币账户。
  // 8. 设置一个代理来进行代币转移或销毁。
  // 9. 通过代理进行代币转移。
  // 10. 撤销代理权限。
  // 11. 销毁一部分代币。
  const connection = new web3.Connection(web3.clusterApiUrl('devnet'))
  const user = await initializeKeypair(connection)

  const mint = await createNewMint(
    connection,
    user,
    user.publicKey,
    user.publicKey,
    2,
  )

  const mintInfo = await token.getMint(connection, mint)

  const tokenAccount = await createTokenAccount(
    connection,
    user,
    mint,
    user.publicKey,
  )

  await mintTokens(
    connection,
    user,
    mint,
    tokenAccount.address,
    user,
    100 * 10 ** mintInfo.decimals,
  )

  const receiver = web3.Keypair.generate().publicKey
  const receiverTokenAccount = await createTokenAccount(
    connection,
    user,
    mint,
    receiver,
  )

  const delegate = web3.Keypair.generate()
  await approveDelegate(
    connection,
    user,
    tokenAccount.address,
    delegate.publicKey,
    user.publicKey,
    50 * 10 ** mintInfo.decimals,
  )

  await transferTokens(
    connection,
    user,
    tokenAccount.address,
    receiverTokenAccount.address,
    delegate,
    50 * 10 ** mintInfo.decimals,
  )

  await revokeDelegate(connection, user, tokenAccount.address, user.publicKey)

  await burnTokens(
    connection,
    user,
    tokenAccount.address,
    mint,
    user,
    25 * 10 ** mintInfo.decimals,
  )
}

/**
 * 创建一个新的代币发行（Mint）。
 */
async function createNewMint(
  connection: web3.Connection,
  payer: web3.Keypair,
  mintAuthority: web3.PublicKey,
  freezeAuthority: web3.PublicKey,
  decimals: number,
): Promise<web3.PublicKey> {
  const tokenMint = await token.createMint(
    connection,
    payer,
    mintAuthority,
    freezeAuthority,
    decimals,
  )

  console.log(
    `Token Mint: https://explorer.solana.com/address/${tokenMint}?cluster=devnet`,
  )

  return tokenMint
}

/**
 * 为指定的代币发行创建一个代币账户。
 */
async function createTokenAccount(
  connection: web3.Connection,
  payer: web3.Keypair,
  mint: web3.PublicKey,
  owner: web3.PublicKey,
) {
  const tokenAccount = await token.getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    mint,
    owner,
  )

  console.log(
    `Token Account: https://explorer.solana.com/address/${tokenAccount.address}?cluster=devnet`,
  )

  return tokenAccount
}

/**
 * 向指定的代币账户铸造代币。
 */
async function mintTokens(
  connection: web3.Connection,
  payer: web3.Keypair,
  mint: web3.PublicKey,
  destination: web3.PublicKey,
  authority: web3.Keypair,
  amount: number,
) {
  const transactionSignature = await token.mintTo(
    connection,
    payer,
    mint,
    destination,
    authority,
    amount,
  )

  console.log(
    `Mint Token Transaction: https://explorer.solana.com/tx/${transactionSignature}?cluster=devnet`,
  )
}

/**
 * 授权代理进行代币转移或销毁。
 */
async function approveDelegate(
  connection: web3.Connection,
  payer: web3.Keypair,
  account: web3.PublicKey,
  delegate: web3.PublicKey,
  owner: web3.Signer | web3.PublicKey,
  amount: number,
) {
  const transactionSignature = await token.approve(
    connection,
    payer,
    account,
    delegate,
    owner,
    amount,
  )

  console.log(
    `Approve Delegate Transaction: https://explorer.solana.com/tx/${transactionSignature}?cluster=devnet`,
  )
}

/**
 * 通过代理进行代币转移。
 */
async function transferTokens(
  connection: web3.Connection,
  payer: web3.Keypair,
  source: web3.PublicKey,
  destination: web3.PublicKey,
  owner: web3.Keypair,
  amount: number,
) {
  const transactionSignature = await token.transfer(
    connection,
    payer,
    source,
    destination,
    owner,
    amount,
  )

  console.log(
    `Transfer Transaction: https://explorer.solana.com/tx/${transactionSignature}?cluster=devnet`,
  )
}

/**
 * 撤销代理权限。
 */
async function revokeDelegate(
  connection: web3.Connection,
  payer: web3.Keypair,
  account: web3.PublicKey,
  owner: web3.Signer | web3.PublicKey,
) {
  const transactionSignature = await token.revoke(connection, payer, account, owner)

  console.log(
    `Revote Delegate Transaction: https://explorer.solana.com/tx/${transactionSignature}?cluster=devnet`,
  )
}

/**
 * 销毁代币。
 */
async function burnTokens(
  connection: web3.Connection,
  payer: web3.Keypair,
  account: web3.PublicKey,
  mint: web3.PublicKey,
  owner: web3.Keypair,
  amount: number,
) {
  const transactionSignature = await token.burn(
    connection,
    payer,
    account,
    mint,
    owner,
    amount,
  )

  console.log(
    `Burn Transaction: https://explorer.solana.com/tx/${transactionSignature}?cluster=devnet`,
  )
}

export default main
