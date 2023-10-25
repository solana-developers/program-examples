import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TransferTokens } from "../target/types/transfer_tokens";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { Metaplex } from "@metaplex-foundation/js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("Transfer Tokens", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const dataAccount = anchor.web3.Keypair.generate();
  const mintKeypair = anchor.web3.Keypair.generate();
  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  const program = anchor.workspace.TransferTokens as Program<TransferTokens>;

  const nftTitle = "Homer NFT";
  const nftSymbol = "HOMR";
  const nftUri =
    "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json";

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .new()
      .accounts({ dataAccount: dataAccount.publicKey })
      .signers([dataAccount])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Create an SPL Token!", async () => {
    const metaplex = Metaplex.make(connection);
    const metadataAddress = await metaplex
      .nfts()
      .pdas()
      .metadata({ mint: mintKeypair.publicKey });

    // Add your test here.
    const tx = await program.methods
      .createTokenMint(
        wallet.publicKey, // freeze authority
        9, // 0 decimals for NFT
        nftTitle, // NFT name
        nftSymbol, // NFT symbol
        nftUri // NFT URI
      )
      .accounts({ 
        payer: wallet.publicKey,
        mint: mintKeypair.publicKey,
        metadata: metadataAddress,
        mintAuthority: wallet.publicKey,
        rentAddress: SYSVAR_RENT_PUBKEY,
        metadataProgramId: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
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

  it("Transfer some tokens to another wallet!", async () => {
    // Wallet's associated token account address for mint
    const tokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer, // payer
      mintKeypair.publicKey, // mint
      wallet.publicKey // owner
    );

    const receipient = anchor.web3.Keypair.generate();
    const receipientTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer, // payer
      mintKeypair.publicKey, // mint account
      receipient.publicKey // owner account
    );

    const tx = await program.methods
      .transferTokens(
        new anchor.BN(150)
      )
      .accounts({ 
        from: tokenAccount.address,
        to: receipientTokenAccount.address,
        owner: wallet.publicKey,
       })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
