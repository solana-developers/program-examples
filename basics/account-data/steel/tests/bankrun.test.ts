import { describe, it } from "mocha";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { assert } from "chai";
import { BanksClient, ProgramTestContext, start } from "solana-bankrun";
import * as borsh from "borsh";
import { decodeString, encodeString } from "./utils";

const PROGRAM_ID = new PublicKey("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

// Define DataAccount type for deserialization
type DataAccount = {
  data: {
    name: string;
    house_number: number;
    street: string;
    city: string;
  };
};

// Define DataAccountRaw type for deserialization
type DataAccountRaw = {
  data: {
    name: Uint8Array;
    house_number: number;
    street: Uint8Array;
    city: Uint8Array;
  };
};

// Define the schema for the account data
const counterAccountSchema: borsh.Schema = {
  struct: {
    discriminator: "u64",
    data: {
      struct: {
        name: { array: { type: "u8", len: 64 } },
        house_number: "u8",
        street: { array: { type: "u8", len: 64 } },
        city: { array: { type: "u8", len: 64 } },
      },
    },
  },
};

// Define the functions to deserialize the account data read from the account
const deserializeDataAccount = (data: DataAccountRaw["data"]): DataAccount => {
  return {
    data: {
      name: decodeString(data.name),
      house_number: data.house_number,
      street: decodeString(data.street),
      city: decodeString(data.city),
    },
  };
};

// Define the function to serialize the get instruction data for the Initialize instruction
const getInitializeInstructionData = (data: DataAccount["data"]): Buffer => {
  const name = encodeString(data.name, 64);
  const house_number = Buffer.from([data.house_number]);
  const street = encodeString(data.street, 64);
  const city = encodeString(data.city, 64);
  return Buffer.concat([
    instructionDiscriminators.Initialize, // discriminator instruction
    name,
    house_number,
    street,
    city,
  ]);
};

// Define the instruction discriminators
const instructionDiscriminators = {
  Initialize: Buffer.from([0]),
};

describe("Account Data Program", () => {
  let context: ProgramTestContext;
  let client: BanksClient;
  let payer: Keypair;
  let dataAcountPda: PublicKey;

  before(async () => {
    context = await start(
      [{ name: "account_data_program", programId: PROGRAM_ID }],
      []
    );
    client = context.banksClient;
    payer = context.payer;

    [dataAcountPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("account"), payer.publicKey.toBuffer()],
      PROGRAM_ID
    );
  });

  it("Should create the address info account successfully", async () => {
    // declare the data to be stored in the account
    const addressInfoData: DataAccount = {
      data: {
        name: "Joe C",
        house_number: 136,
        street: "Mile High Dr.",
        city: "Solana Beach",
      },
    };

    // send the transaction
    const tx = new Transaction();
    tx.add(
      new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: dataAcountPda, isSigner: false, isWritable: true },
          {
            pubkey: SystemProgram.programId,
            isSigner: false,
            isWritable: false,
          },
        ],
        data: getInitializeInstructionData(addressInfoData["data"]),
      })
    );
    tx.recentBlockhash = context.lastBlockhash;
    tx.sign(payer);

    // process the transaction
    await client.processTransaction(tx);

    const account = await client.getAccount(dataAcountPda);
    assert.isNotNull(account);
    const rawAccountData = borsh.deserialize(
      counterAccountSchema,
      account?.data
    ) as DataAccountRaw;

    const deserializedData = deserializeDataAccount(rawAccountData["data"]);
    assert.deepEqual(deserializedData, addressInfoData);
  });
});
