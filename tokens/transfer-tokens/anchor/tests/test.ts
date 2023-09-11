import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
import * as anchor from "@coral-xyz/anchor";
import { TransferTokens } from "../target/types/transfer_tokens";
import {
  PublicKey,
  Keypair,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
} from "@solana/web3.js";
import {
  getAssociatedTokenAddressSync,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("Transfer Tokens", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace
    .TransferTokens as anchor.Program<TransferTokens>;

  const metadata = {
    name: "Solana Gold",
    symbol: "GOLDSOL",
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
  };

  // Generate new keypair to use as address for mint account.
  const mintKeypair = new Keypair();

  // Generate new keypair to use as address for recipient wallet.
  const recipient = new Keypair();

  // Derive the associated token address account for the mint and payer.
  const senderTokenAddress = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    payer.publicKey
  );

  // Derive the associated token address account for the mint and recipient.
  const recepientTokenAddress = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    recipient.publicKey
  );

  it("Create an SPL Token!", async () => {
    // Derive the metadata account address.
    const [metadataAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    const transactionSignature = await program.methods
      .createToken(metadata.name, metadata.symbol, metadata.uri)
      .accounts({
        payer: payer.publicKey,
        mintAccount: mintKeypair.publicKey,
        metadataAccount: metadataAddress,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .signers([mintKeypair])
      .rpc();

    console.log("Success!");
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });

  it("Mint tokens!", async () => {
    // Amount of tokens to mint.
    const amount = new anchor.BN(100);

    // Mint the tokens to the associated token account.
    const transactionSignature = await program.methods
      .mintToken(amount)
      .accounts({
        mintAuthority: payer.publicKey,
        recepient: payer.publicKey,
        mintAccount: mintKeypair.publicKey,
        associatedTokenAccount: senderTokenAddress,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Success!");
    console.log(`   Associated Token Account Address: ${senderTokenAddress}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });

  it("Transfer tokens!", async () => {
    // Amount of tokens to transfer.
    const amount = new anchor.BN(50);

    const transactionSignature = await program.methods
      .transferTokens(amount)
      .accounts({
        sender: payer.publicKey,
        recipient: recipient.publicKey,
        mintAccount: mintKeypair.publicKey,
        senderTokenAccount: senderTokenAddress,
        recipientTokenAccount: recepientTokenAddress,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Success!");
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });
});
