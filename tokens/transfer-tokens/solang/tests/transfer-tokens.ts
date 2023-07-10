import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { TransferTokens } from "../target/types/transfer_tokens"
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js"
import { Metaplex } from "@metaplex-foundation/js"
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token"

describe("Transfer Tokens", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const dataAccount = anchor.web3.Keypair.generate()
  const mintKeypair = anchor.web3.Keypair.generate()
  const wallet = provider.wallet as anchor.Wallet
  const connection = provider.connection

  const program = anchor.workspace.TransferTokens as Program<TransferTokens>

  const nftTitle = "Homer NFT"
  const nftSymbol = "HOMR"
  const nftUri =
    "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json"

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .new(wallet.publicKey)
      .accounts({ dataAccount: dataAccount.publicKey })
      .signers([dataAccount])
      .rpc()
    console.log("Your transaction signature", tx)
  })

  it("Create an SPL Token!", async () => {
    const metaplex = Metaplex.make(connection)
    const metadataAddress = await metaplex
      .nfts()
      .pdas()
      .metadata({ mint: mintKeypair.publicKey })

    // Add your test here.
    const tx = await program.methods
      .createTokenMint(
        wallet.publicKey, // payer
        mintKeypair.publicKey, // mint
        wallet.publicKey, // mint authority
        wallet.publicKey, // freeze authority
        metadataAddress, // metadata address
        9, // 0 decimals for NFT
        nftTitle, // NFT name
        nftSymbol, // NFT symbol
        nftUri // NFT URI
      )
      .accounts({ dataAccount: dataAccount.publicKey })
      .remainingAccounts([
        {
          pubkey: wallet.publicKey,
          isWritable: true,
          isSigner: true,
        },
        { pubkey: mintKeypair.publicKey, isWritable: true, isSigner: true },
        {
          pubkey: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
          isWritable: false,
          isSigner: false,
        },
        { pubkey: metadataAddress, isWritable: true, isSigner: false },
        { pubkey: SystemProgram.programId, isWritable: false, isSigner: false },
        { pubkey: SYSVAR_RENT_PUBKEY, isWritable: false, isSigner: false },
      ])
      .signers([mintKeypair])
      .rpc({ skipPreflight: true })
    console.log("Your transaction signature", tx)
  })

  it("Mint some tokens to your wallet!", async () => {
    // Wallet's associated token account address for mint
    const tokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer, // payer
      mintKeypair.publicKey, // mint
      wallet.publicKey // owner
    )

    const tx = await program.methods
      .mintTo(
        wallet.publicKey, // payer
        tokenAccount.address, // associated token account address
        mintKeypair.publicKey, // mint
        wallet.publicKey, // owner of token account
        new anchor.BN(150) // amount to mint
      )
      .accounts({ dataAccount: dataAccount.publicKey })
      .remainingAccounts([
        {
          pubkey: wallet.publicKey,
          isWritable: true,
          isSigner: true,
        },
        { pubkey: tokenAccount.address, isWritable: true, isSigner: false },
        { pubkey: mintKeypair.publicKey, isWritable: true, isSigner: false },
        {
          pubkey: SystemProgram.programId,
          isWritable: false,
          isSigner: false,
        },
        { pubkey: TOKEN_PROGRAM_ID, isWritable: false, isSigner: false },
        {
          pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
          isWritable: false,
          isSigner: false,
        },
      ])
      .rpc({ skipPreflight: true })
    console.log("Your transaction signature", tx)
  })

  it("Transfer some tokens to another wallet!", async () => {
    // Wallet's associated token account address for mint
    const tokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer, // payer
      mintKeypair.publicKey, // mint
      wallet.publicKey // owner
    )

    const receipient = anchor.web3.Keypair.generate()
    const receipientTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer, // payer
      mintKeypair.publicKey, // mint account
      receipient.publicKey // owner account
    )

    const tx = await program.methods
      .transferTokens(
        tokenAccount.address,
        receipientTokenAccount.address,
        new anchor.BN(150)
      )
      .accounts({ dataAccount: dataAccount.publicKey })
      .remainingAccounts([
        {
          pubkey: wallet.publicKey,
          isWritable: true,
          isSigner: true,
        },
        {
          pubkey: mintKeypair.publicKey,
          isWritable: false,
          isSigner: false,
        },
        {
          pubkey: tokenAccount.address,
          isWritable: true,
          isSigner: false,
        },
        {
          pubkey: receipientTokenAccount.address,
          isWritable: true,
          isSigner: false,
        },
      ])
      .rpc()
    console.log("Your transaction signature", tx)
  })
})
