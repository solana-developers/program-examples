import { describe, test } from "node:test";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionInstruction } from "@solana/web3.js";
import { start } from "solana-bankrun";

interface AddressInfo {
  name: string;
  house_number: number;
  street: string;
  city: string;
}

function toBytes(addressInfo: AddressInfo): Buffer {
  const data: number[] = [];

  // Add instruction discriminator
  data.push(0);

  // Pad name to 32 bytes
  const nameBytes = Buffer.from(addressInfo.name, "utf-8");
  const namePadded = Buffer.alloc(32);
  nameBytes.copy(namePadded, 0, 0, Math.min(nameBytes.length, 32));
  data.push(...namePadded);

  // Add house_number
  data.push(addressInfo.house_number);

  // Pad street to 32 bytes
  const streetBytes = Buffer.from(addressInfo.street, "utf-8");
  const streetPadded = Buffer.alloc(32);
  streetBytes.copy(streetPadded, 0, 0, Math.min(streetBytes.length, 32));
  data.push(...streetPadded);

  // Pad city to 32 bytes
  const cityBytes = Buffer.from(addressInfo.city, "utf-8");
  const cityPadded = Buffer.alloc(32);
  cityBytes.copy(cityPadded, 0, 0, Math.min(cityBytes.length, 32));
  data.push(...cityPadded);

  return Buffer.from(data);
}

function fromBytes(buffer: Buffer): AddressInfo {
  // Skip discriminator (first 8 bytes)
  const offset = 8;

  // name: bytes 0..32
  const nameBytes = buffer.subarray(offset, offset + 32);
  const name = nameBytes.toString("utf-8").replace(/\0/g, "");

  // house_number: byte 32
  const house_number = buffer[offset + 32];

  // street: bytes 33..65
  const streetBytes = buffer.subarray(offset + 33, offset + 65);
  const street = streetBytes.toString("utf-8").replace(/\0/g, "");

  // city: bytes 65..97
  const cityBytes = buffer.subarray(offset + 65, offset + 97);
  const city = cityBytes.toString("utf-8").replace(/\0/g, "");

  return { name, house_number, street, city };
}

describe("Account Data (steel)", async () => {
  const PROGRAM_ID = PublicKey.unique();
  const context = await start([{ name: "account_data_steel_program", programId: PROGRAM_ID }], []);
  const client = context.banksClient;
  const payer = context.payer;

  const addressInfoAccount = Keypair.generate();

  test("Create the address info account", async () => {
    console.log(`Program Address    : ${PROGRAM_ID}`);
    console.log(`Payer Address      : ${payer.publicKey}`);
    console.log(`Address Info Acct  : ${addressInfoAccount.publicKey}`);

    const addressInfo: AddressInfo = {
      name: "Joe C",
      house_number: 136,
      street: "Mile High Dr.",
      city: "Solana Beach",
    };

    const ix = new TransactionInstruction({
      keys: [
        {
          pubkey: addressInfoAccount.publicKey,
          isSigner: true,
          isWritable: true,
        },
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: toBytes(addressInfo),
    });

    const tx = new Transaction();
    const [blockhash, _] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, addressInfoAccount);

    await client.processTransaction(tx);
  });

  test("Read the new account's data", async () => {
    const accountInfo = await client.getAccount(addressInfoAccount.publicKey);

    if (!accountInfo) {
      throw new Error("Account not found");
    }

    const readAddressInfo = fromBytes(Buffer.from(accountInfo.data));

    console.log(`Name     : ${readAddressInfo.name}`);
    console.log(`House Num: ${readAddressInfo.house_number}`);
    console.log(`Street   : ${readAddressInfo.street}`);
    console.log(`City     : ${readAddressInfo.city}`);
  });
});
