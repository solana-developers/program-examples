import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { CreateToken } from "../target/types/create_token"
import { Metaplex } from "@metaplex-foundation/js"
import { SYSVAR_RENT_PUBKEY, SystemProgram, PublicKey } from "@solana/web3.js"

describe("create-token", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const program = anchor.workspace.CreateToken as Program<CreateToken>

  // Generate new keypair to use as data account
  const dataAccount = anchor.web3.Keypair.generate()
  const wallet = provider.wallet
  const connection = provider.connection

  // Metadata for the token
  const tokenTitle = "Solana Gold"
  const tokenSymbol = "GOLDSOL"
  const tokenUri =
    "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json"

  it("Is initialized!", async () => {
    // Initialize data account for the program, which is required by Solang
    const tx = await program.methods
      .new(wallet.publicKey)
      .accounts({ dataAccount: dataAccount.publicKey })
      .signers([dataAccount])
      .rpc()
    console.log("Your transaction signature", tx)
  })

  it("Create an SPL Token!", async () => {
    // Generate a new keypair for the mint
    const mintKeypair = anchor.web3.Keypair.generate()

    // Get the metadata address for the mint
    const metaplex = Metaplex.make(connection)
    const metadataAddress = await metaplex
      .nfts()
      .pdas()
      .metadata({ mint: mintKeypair.publicKey })

    const tx = await program.methods
      .createTokenMint(
        wallet.publicKey, // payer
        mintKeypair.publicKey, // mint
        wallet.publicKey, // freeze authority
        wallet.publicKey, // mint authority
        metadataAddress, // metadata address
        9, // decimals for the mint
        tokenTitle, // token name
        tokenSymbol, // token symbol
        tokenUri // token uri (off-chain metadata)
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
          pubkey: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"), // Metadata program id
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

  it("Create an NFT!", async () => {
    // Generate a new keypair for the mint
    const mintKeypair = anchor.web3.Keypair.generate()

    // Get the metadata address for the mint
    const metaplex = Metaplex.make(connection)
    const metadataAddress = await metaplex
      .nfts()
      .pdas()
      .metadata({ mint: mintKeypair.publicKey })

    const tx = await program.methods
      .createTokenMint(
        wallet.publicKey,
        mintKeypair.publicKey,
        wallet.publicKey,
        wallet.publicKey,
        metadataAddress,
        0, // decimals for the mint, 0 for NFTs "non-fungible"
        tokenTitle,
        tokenSymbol,
        tokenUri
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
          pubkey: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"), // Metadata program id
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
})
