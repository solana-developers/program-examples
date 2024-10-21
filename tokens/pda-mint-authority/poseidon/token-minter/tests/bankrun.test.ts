import { describe, it } from "node:test";
import * as anchor from "@coral-xyz/anchor";
import { createInitializeMintInstruction, createMint, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { Connection, PublicKey, SystemProgram, Transaction, TransactionInstruction } from "@solana/web3.js";
import { BankrunProvider } from "anchor-bankrun";
import { startAnchor } from "solana-bankrun";
import type { TokenMinter } from "../target/types/token_minter";
import { createV1, TokenStandard } from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, createUmi, generateSigner, percentAmount } from "@metaplex-foundation/umi";

const IDL = require("../target/idl/token_minter.json");
const PROGRAM_ID = new PublicKey(IDL.address);
const METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

describe("NFT Minter", async () => {
  const context = await startAnchor(
    "",
    [
      { name: "token_minter", programId: PROGRAM_ID },
      { name: "token_metadata", programId: METADATA_PROGRAM_ID },
    ],
    []
  );
  const provider = new BankrunProvider(context);
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<TokenMinter>(IDL, provider);

  // Derive the PDA to use as mint account address.
  // This same PDA is also used as the mint authority.
  const [mintPDA, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    program.programId
  );

  const metadata = {
    name: "Solana Gold",
    symbol: "GOLDSOL",
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
  };
  
  const connection = new Connection("https://api.devnet.solana.com.");
// const umi = createUmi()
// Token has to be 
//   it("Create a token!", async () => {
//     const transactionSignature = await program.methods
//       .createToken(metadata.name, metadata.symbol, metadata.uri)
//       .accounts({
//         payer: payer.publicKey,
//       })
//       .rpc();

//     console.log("Success!");
//     console.log(`   Mint Address: ${mintPDA}`);
//     console.log(`   Transaction Signature: ${transactionSignature}`);
//   });
  const transaction = new Transaction();

  // Create the account for the SPL Token Mint
  transaction.add(
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: mintPDA, // The mint account public key
      space: 1000, // Size of the mint account
      lamports: await connection.getMinimumBalanceForRentExemption(1000), // Rent-exempt balance for mint
      programId: PROGRAM_ID, // Program ID of the SPL Token program
    })
  );

  // Add the initialize mint instruction
  transaction.add(
    createInitializeMintInstruction(
      mintPDA, // The mint account pda
      9, // Decimals (for example, 9 for 1 billionth precision like SOL)
      mintPDA, // Mint authority set to the PDA
      mintPDA, // Freeze authority set to the PDA (optional)
      PROGRAM_ID // Program ID of the SPL Token program
    )
  );


// createMint(
//   connection,

//    // Decimals (for example, 9 for 1 billionth precision like SOL)
//   mintPDA, // Mint authority set to the PDA
//   mintPDA,
//   9, // Freeze authority set to the PDA (optional)
//   PROGRAM_ID // Program ID of the SPL Token program
// )

const instruction = new TransactionInstruction({
  keys: [
    { pubkey: mintPDA, isSigner: true, isWritable: true }, // PDA as signer
    { pubkey: payer.publicKey, isSigner: true, isWritable: true },
  ],
  programId: program.programId, // The program you are interacting with
  data: Buffer.from([bump]), // Program will use the bump to validate the PDA
});

transaction.add(instruction)



// const mint = generateSigner(umi);

// await createV1(umi, {
//   mint,
//   authority:mintPDA,
//   name: metadata.name,
//   symbol: metadata.symbol,
//   uri:metadata.uri,
//   sellerFeeBasisPoints: percentAmount(5.5),
//   tokenStandard: TokenStandard.Fungible,
// }).sendAndConfirm(umi);

  it("Mint 1 Token!", async () => {
    // Derive the associated token address account for the mint and payer.
    const associatedTokenAccountAddress = getAssociatedTokenAddressSync(
      mintPDA,
      payer.publicKey
    );

    // Amount of tokens to mint.
    const amount = new anchor.BN(100);

    const transactionSignature = await program.methods
      .mintToken(amount)
      .accounts({
        payer: payer.publicKey,
        associatedTokenAccount: associatedTokenAccountAddress,
      })
      .rpc();

    console.log("Success!");
    console.log(
      `   Associated Token Account Address: ${associatedTokenAccountAddress}`
    );
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });
});
