import { Buffer } from "node:buffer";
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from "@solana/web3.js";
import * as borsh from "borsh";
import { assert } from "chai";
import { start } from "solana-bankrun";

// The legacy SPL Token and Associated Token Account programs are bundled with
// bankrun. The Metaplex Token Metadata program is not, so it is dumped from
// mainnet into tests/fixtures by prepare.mjs and loaded by name below.
const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
const TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

// Instruction discriminators (the Borsh enum variant index).
const CREATE = 0;
const MINT = 1;

// Borsh schema for the Create instruction data, matching the program's
// `CreateTokenArgs` (and the native example's wire format).
const CreateTokenArgsSchema: borsh.Schema = {
  struct: {
    instruction: "u8",
    token_title: "string",
    token_symbol: "string",
    token_uri: "string",
  },
};

// Encode a u64 as 8 little-endian bytes without relying on BigInt (tsconfig
// targets es6). Safe for amounts below 2^53.
function u64le(value: number): Buffer {
  const buffer = Buffer.alloc(8);
  buffer.writeUInt32LE(value >>> 0, 0);
  buffer.writeUInt32LE(Math.floor(value / 4294967296), 4); // 4294967296 = 2^32
  return buffer;
}

function getMetadataAddress(mint: PublicKey): PublicKey {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("metadata"), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    TOKEN_METADATA_PROGRAM_ID,
  )[0];
}

function getAssociatedTokenAddress(mint: PublicKey, owner: PublicKey): PublicKey {
  return PublicKey.findProgramAddressSync(
    [owner.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  )[0];
}

// Read the `amount` field (u64 at offset 64) of an SPL token account.
function readTokenAmount(data: Uint8Array): number {
  const buffer = Buffer.from(data);
  return buffer.readUInt32LE(64) + buffer.readUInt32LE(68) * 4294967296; // 2^32
}

describe("SPL Token Minter (Pinocchio)", async () => {
  const PROGRAM_ID = PublicKey.unique();
  const context = await start(
    [
      { name: "spl_token_minter_pinocchio_program", programId: PROGRAM_ID },
      { name: "token_metadata", programId: TOKEN_METADATA_PROGRAM_ID },
    ],
    [],
  );
  const client = context.banksClient;
  const payer = context.payer;

  const mintKeypair = Keypair.generate();

  async function sendInstruction(ix: TransactionInstruction, signers: Keypair[]) {
    const tx = new Transaction();
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix);
    tx.sign(...signers);
    await client.processTransaction(tx);
  }

  it("Create an SPL Token!", async () => {
    const metadataAddress = getMetadataAddress(mintKeypair.publicKey);

    const data = Buffer.from(
      borsh.serialize(CreateTokenArgsSchema, {
        instruction: CREATE,
        token_title: "Solana Gold",
        token_symbol: "GOLDSOL",
        token_uri:
          "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
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

    await sendInstruction(ix, [payer, mintKeypair]);

    const mintAccount = await client.getAccount(mintKeypair.publicKey);
    if (mintAccount === null) throw new Error("Mint account not found");
    assert.deepEqual(mintAccount.owner.toBytes(), TOKEN_PROGRAM_ID.toBytes());

    const metadataAccount = await client.getAccount(metadataAddress);
    if (metadataAccount === null) throw new Error("Metadata account not found");
    assert.deepEqual(metadataAccount.owner.toBytes(), TOKEN_METADATA_PROGRAM_ID.toBytes());
    assert.isTrue(Buffer.from(metadataAccount.data).toString("utf-8").includes("Solana Gold"));
  });

  it("Mint some tokens to your wallet!", async () => {
    const ata = getAssociatedTokenAddress(mintKeypair.publicKey, payer.publicKey);

    const ix = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        { pubkey: mintKeypair.publicKey, isSigner: false, isWritable: true }, // mint account
        { pubkey: payer.publicKey, isSigner: false, isWritable: false }, // mint authority
        { pubkey: ata, isSigner: false, isWritable: true }, // associated token account
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // payer
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // system program
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // token program
        { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // associated token program
      ],
      data: Buffer.concat([Buffer.from([MINT]), u64le(150)]),
    });

    await sendInstruction(ix, [payer]);

    const ataAccount = await client.getAccount(ata);
    if (ataAccount === null) throw new Error("Associated token account not found");
    assert.equal(readTokenAmount(ataAccount.data), 150);
  });
});
