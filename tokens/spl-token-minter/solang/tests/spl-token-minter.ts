import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SplTokenMinter } from "../target/types/spl_token_minter";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { Metaplex } from "@metaplex-foundation/js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("spl-token-minter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Generate a new keypair for the data account for the program
  const dataAccount = anchor.web3.Keypair.generate();
  // Generate a mint keypair
  const mintKeypair = anchor.web3.Keypair.generate();
  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  const program = anchor.workspace.SplTokenMinter as Program<SplTokenMinter>;

  // Metadata for the Token
  const tokenTitle = "Solana Gold";
  const tokenSymbol = "GOLDSOL";
  const tokenUri =
    "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json";

  it("Is initialized!", async () => {
    // Initialize data account for the program, which is required by Solang
    const tx = await program.methods
      .new()
      .accounts({ dataAccount: dataAccount.publicKey })
      .signers([dataAccount])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Create an SPL Token!", async () => {
    // Get the metadata address for the mint
    const metaplex = Metaplex.make(connection);
    const metadataAddress = await metaplex
      .nfts()
      .pdas()
      .metadata({ mint: mintKeypair.publicKey });

    // Create the token mint
    const tx = await program.methods
      .createTokenMint(
        wallet.publicKey, // freeze authority
        9, // decimals
        tokenTitle, // token name
        tokenSymbol, // token symbol
        tokenUri // token uri
      )
      .accounts({ 
        payer: wallet.publicKey,
        mint: mintKeypair.publicKey,
        metadata: metadataAddress,
        mintAuthority: wallet.publicKey,
        rentAddress: SYSVAR_RENT_PUBKEY,
        metadataProgramId: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
       })
      .signers([mintKeypair])
      .rpc({ skipPreflight: true });
    console.log("Your transaction signature", tx);
  });

  it("Mint some tokens to your wallet!", async () => {
    // Wallet's associated token account address for mint
    const tokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer, // payer
      mintKeypair.publicKey, // mint
      wallet.publicKey // owner
    );

    const tx = await program.methods
      .mintTo(
        new anchor.BN(150) // amount to mint
      )
      .accounts({ 
        mintAuthority: wallet.publicKey,
        tokenAccount: tokenAccount.address,
        mint: mintKeypair.publicKey,
       })
      .rpc({ skipPreflight: true });
    console.log("Your transaction signature", tx);
  });
});
