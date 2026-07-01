import { Buffer } from "node:buffer";
import {
  Keypair,
  PublicKey,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import * as borsh from "borsh";
import { assert } from "chai";
import { start } from "solana-bankrun";

// The Token-2022 program is bundled with bankrun, so there is no fixture to
// load. Its ID is hard-coded here to avoid pulling in @solana/spl-token.
const TOKEN_2022_PROGRAM_ID = new PublicKey("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

// Borsh schema for the instruction data, matching the program's
// `CreateTokenArgs` (and the native example's wire format).
const CreateTokenArgsSchema: borsh.Schema = {
  struct: { token_decimals: "u8" },
};

// Token-2022 lays a mint with one extension out as:
//   base account length (165) + account-type byte (1) + TLV entry (36) = 202
const EXTENDED_MINT_SIZE = 202;
const ACCOUNT_TYPE_OFFSET = 165; // 1 == Mint
const TLV_TYPE_OFFSET = 166; // u16 LE, 3 == MintCloseAuthority
const TLV_VALUE_OFFSET = 170; // 32-byte close authority pubkey
const DECIMALS_OFFSET = 44; // in the base mint layout
const MINT_CLOSE_AUTHORITY_EXTENSION = 3;
const ACCOUNT_TYPE_MINT = 1;

describe("Token-2022 Mint Close Authority (Pinocchio)", async () => {
  const PROGRAM_ID = PublicKey.unique();
  const context = await start(
    [{ name: "token_2022_mint_close_authority_pinocchio_program", programId: PROGRAM_ID }],
    [],
  );
  const client = context.banksClient;
  const payer = context.payer;

  it("Creates a Token-2022 mint with a close authority", async () => {
    const decimals = 9;
    const mintKeypair = Keypair.generate();

    const data = Buffer.from(borsh.serialize(CreateTokenArgsSchema, { token_decimals: decimals }));

    const ix = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        { pubkey: mintKeypair.publicKey, isSigner: true, isWritable: true }, // mint account
        { pubkey: payer.publicKey, isSigner: false, isWritable: false }, // mint authority
        { pubkey: payer.publicKey, isSigner: false, isWritable: false }, // close authority
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // payer
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false }, // rent sysvar
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // system program
        { pubkey: TOKEN_2022_PROGRAM_ID, isSigner: false, isWritable: false }, // Token-2022 program
      ],
      data,
    });

    const tx = new Transaction();
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix);
    tx.sign(payer, mintKeypair);
    await client.processTransaction(tx);

    const mintAccount = await client.getAccount(mintKeypair.publicKey);
    if (mintAccount === null) throw new Error("Mint account not found");
    const mintData = Buffer.from(mintAccount.data);

    // Owned by Token-2022, and sized for exactly one extension.
    assert.deepEqual(mintAccount.owner.toBytes(), TOKEN_2022_PROGRAM_ID.toBytes());
    assert.equal(mintData.length, EXTENDED_MINT_SIZE);

    // Base mint fields were initialized.
    assert.equal(mintData[DECIMALS_OFFSET], decimals);

    // The extension header marks this as a Mint carrying MintCloseAuthority.
    assert.equal(mintData[ACCOUNT_TYPE_OFFSET], ACCOUNT_TYPE_MINT);
    assert.equal(mintData.readUInt16LE(TLV_TYPE_OFFSET), MINT_CLOSE_AUTHORITY_EXTENSION);

    // The configured close authority was stored in the extension.
    const storedCloseAuthority = mintData.subarray(TLV_VALUE_OFFSET, TLV_VALUE_OFFSET + 32);
    assert.deepEqual(new Uint8Array(storedCloseAuthority), payer.publicKey.toBytes());

    console.log("Mint address:", mintKeypair.publicKey.toBase58());
  });
});
