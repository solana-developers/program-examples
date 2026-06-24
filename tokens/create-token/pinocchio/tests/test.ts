import { Buffer } from "node:buffer";
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from "@solana/web3.js";
import * as borsh from "borsh";
import { assert } from "chai";
import { start } from "solana-bankrun";

// The legacy SPL Token program is bundled with bankrun. The Metaplex Token
// Metadata program is not, so it is dumped from mainnet into tests/fixtures by
// prepare.mjs and loaded by name below.
const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

// Borsh schema for the instruction data, matching the program's `CreateTokenArgs`
// (and the native example's wire format).
const CreateTokenArgsSchema: borsh.Schema = {
  struct: {
    token_title: "string",
    token_symbol: "string",
    token_uri: "string",
    token_decimals: "u8",
  },
};

// Derive the Metaplex metadata PDA for a mint.
function getMetadataAddress(mint: PublicKey): PublicKey {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("metadata"), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    TOKEN_METADATA_PROGRAM_ID,
  )[0];
}

describe("Create Token (Pinocchio)", async () => {
  const PROGRAM_ID = PublicKey.unique();
  const context = await start(
    [
      { name: "create_token_pinocchio_program", programId: PROGRAM_ID },
      { name: "token_metadata", programId: TOKEN_METADATA_PROGRAM_ID },
    ],
    [],
  );
  const client = context.banksClient;
  const payer = context.payer;

  async function createToken(name: string, symbol: string, uri: string, decimals: number) {
    const mintKeypair = Keypair.generate();
    const metadataAddress = getMetadataAddress(mintKeypair.publicKey);

    const data = Buffer.from(
      borsh.serialize(CreateTokenArgsSchema, {
        token_title: name,
        token_symbol: symbol,
        token_uri: uri,
        token_decimals: decimals,
      }),
    );

    const ix = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        { pubkey: mintKeypair.publicKey, isSigner: true, isWritable: true }, // mint account
        { pubkey: payer.publicKey, isSigner: false, isWritable: false }, // mint authority
        { pubkey: metadataAddress, isSigner: false, isWritable: true }, // metadata account
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // payer
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // system program
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // token program
        { pubkey: TOKEN_METADATA_PROGRAM_ID, isSigner: false, isWritable: false }, // token metadata program
      ],
      data,
    });

    const tx = new Transaction();
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix);
    tx.sign(payer, mintKeypair);
    await client.processTransaction(tx);

    return { mint: mintKeypair.publicKey, metadata: metadataAddress };
  }

  it("Create an SPL Token!", async () => {
    const { mint, metadata } = await createToken(
      "Solana Gold",
      "GOLDSOL",
      "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
      9,
    );

    const mintAccount = await client.getAccount(mint);
    if (mintAccount === null) throw new Error("Mint account not found");
    assert.deepEqual(mintAccount.owner.toBytes(), TOKEN_PROGRAM_ID.toBytes());

    const metadataAccount = await client.getAccount(metadata);
    if (metadataAccount === null) throw new Error("Metadata account not found");
    assert.deepEqual(metadataAccount.owner.toBytes(), TOKEN_METADATA_PROGRAM_ID.toBytes());
    assert.isTrue(Buffer.from(metadataAccount.data).toString("utf-8").includes("Solana Gold"));
  });

  it("Create an NFT!", async () => {
    const { mint, metadata } = await createToken(
      "Homer NFT",
      "HOMR",
      "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json",
      0,
    );

    const mintAccount = await client.getAccount(mint);
    if (mintAccount === null) throw new Error("Mint account not found");
    assert.deepEqual(mintAccount.owner.toBytes(), TOKEN_PROGRAM_ID.toBytes());

    const metadataAccount = await client.getAccount(metadata);
    if (metadataAccount === null) throw new Error("Metadata account not found");
    assert.deepEqual(metadataAccount.owner.toBytes(), TOKEN_METADATA_PROGRAM_ID.toBytes());
    assert.isTrue(Buffer.from(metadataAccount.data).toString("utf-8").includes("Homer NFT"));
  });
});
