import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { PdaMintAuthority } from "../target/types/pda_mint_authority"
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js"
import { Metaplex } from "@metaplex-foundation/js"
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token"

describe("pda-mint-authority", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const mintKeypair = anchor.web3.Keypair.generate()
  const wallet = provider.wallet
  const connection = provider.connection

  const program = anchor.workspace.PdaMintAuthority as Program<PdaMintAuthority>

  // Derive the PDA that will be used to initialize the dataAccount.
  const [dataAccountPDA, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("mint_authority")],
    program.programId
  )

  const nftTitle = "Homer NFT"
  const nftSymbol = "HOMR"
  const nftUri =
    "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json"

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .new(wallet.publicKey, [bump])
      .accounts({ dataAccount: dataAccountPDA })
      .rpc()
    console.log("Your transaction signature", tx)
  })

  it("Create an NFT!", async () => {
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
        dataAccountPDA, // mint authority
        dataAccountPDA, // freeze authority
        metadataAddress, // metadata address
        0, // 0 decimals for NFT
        nftTitle, // NFT name
        nftSymbol, // NFT symbol
        nftUri // NFT URI
      )
      .accounts({ dataAccount: dataAccountPDA })
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

  it("Mint the NFT to your wallet!", async () => {
    // Derive wallet's associated token account address for mint
    const tokenAccount = getAssociatedTokenAddressSync(
      mintKeypair.publicKey,
      wallet.publicKey
    )

    const tx = await program.methods
      .mintTo(
        wallet.publicKey, // payer
        tokenAccount, // associated token account address
        mintKeypair.publicKey, // mint
        wallet.publicKey // owner of token account
      )
      .accounts({ dataAccount: dataAccountPDA })
      .remainingAccounts([
        {
          pubkey: wallet.publicKey,
          isWritable: true,
          isSigner: true,
        },
        { pubkey: mintKeypair.publicKey, isWritable: true, isSigner: false },
        { pubkey: tokenAccount, isWritable: true, isSigner: false },
        { pubkey: dataAccountPDA, isWritable: true, isSigner: false },
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
})
