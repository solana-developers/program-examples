import { Buffer } from "node:buffer";
import { describe, test } from "node:test";
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import {
  Keypair,
  PublicKey,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { BN } from "bn.js";
import { start } from "solana-bankrun";
import { CreateTokenArgs, SplMinterInstruction } from "./instructions";

describe("Create Tokens!", async () => {
  const PROGRAM_ID = new PublicKey(
    "z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35",
  );
  const context = await start(
    [
      { name: "create_token_program", programId: PROGRAM_ID },
      { name: "token_metadata", programId: TOKEN_METADATA_PROGRAM_ID },
    ],
    [],
  );
  const client = context.banksClient;
  const payer = context.payer;

  const tokenMintKeypair: Keypair = Keypair.generate();
  const nftMintKeypair: Keypair = Keypair.generate();

  test("Create an SPL Token!", async () => {
    const metadataPDA = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        tokenMintKeypair.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];

    // SPL Token default = 9 decimals
    //
    const createArgs = new CreateTokenArgs(
      "Solana Gold",
      "GOLDSOL",
      "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
      "9",
    );

    const createTokenIx = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        {
          pubkey: tokenMintKeypair.publicKey,
          isSigner: true,
          isWritable: true,
        },
        { pubkey: metadataPDA, isSigner: false, isWritable: true },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        {
          pubkey: TOKEN_METADATA_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        },
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: createArgs.toBuffer(),
    });

    const tx = new Transaction();
    const [blockhash, _] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(createTokenIx).sign(payer, tokenMintKeypair);

    await client.processTransaction(tx);

    console.log("Success!");
    console.log(`   Mint Address: ${tokenMintKeypair.publicKey}`);
  });

  test("Create an NFT!", async () => {
    const metadataPDA = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        nftMintKeypair.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];

    // NFT default = 0 decimals
    //
    const createArgs = new CreateTokenArgs(
      "Homer NFT",
      "HOMR",
      "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json",
      "0",
    );

    const createTokenIx = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: nftMintKeypair.publicKey, isSigner: true, isWritable: true },
        { pubkey: metadataPDA, isSigner: false, isWritable: true },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        {
          pubkey: TOKEN_METADATA_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        },
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: createArgs.toBuffer(),
    });

    const tx = new Transaction();
    const [blockhash, _] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(createTokenIx).sign(payer, nftMintKeypair);

    await client.processTransaction(tx);

    console.log("Success!");
    console.log(`   Mint Address: ${nftMintKeypair.publicKey}`);
  });
});
