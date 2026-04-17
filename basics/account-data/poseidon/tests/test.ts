import { describe, test, beforeAll } from "@jest/globals";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { BankrunProvider } from "anchor-bankrun";
import { startAnchor } from "solana-bankrun";
import { Program } from "@coral-xyz/anchor";

const IDL = require("../program/target/idl/account_data_poseidon_program.json");
const PROGRAM_ID = new PublicKey(IDL.address);

describe("Account Data (Poseidon)", () => {
  let context: any;
  let provider: BankrunProvider;
  let program: Program;
  let owner: Keypair;

  beforeAll(async () => {
    owner = Keypair.generate();

    context = await startAnchor(
      "",
      [{ name: "account_data_poseidon_program", programId: PROGRAM_ID }],
      [
        {
          address: owner.publicKey,
          info: {
            lamports: 10 * LAMPORTS_PER_SOL,
            data: Buffer.alloc(0),
            owner: SystemProgram.programId,
            executable: false,
          },
        },
      ]
    );

    provider = new BankrunProvider(context);
    program = new Program(IDL, provider);
  });

  test("Create address info", async () => {
    const [addressInfoPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("address_info"), owner.publicKey.toBuffer()],
      PROGRAM_ID
    );

    const name = "Alice";
    const houseNumber = 42;
    const street = "Main Street";
    const city = "San Francisco";

    await program.methods
      .createAddressInfo(name, houseNumber, street, city)
      .accounts({
        owner: owner.publicKey,
        addressInfo: addressInfoPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([owner])
      .rpc();

    const addressInfo = await program.account.addressInfo.fetch(addressInfoPda);

    expect(addressInfo.name).toBe(name);
    expect(addressInfo.houseNumber).toBe(houseNumber);
    expect(addressInfo.street).toBe(street);
    expect(addressInfo.city).toBe(city);
  });
});
