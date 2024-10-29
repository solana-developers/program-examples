import {
    type Blockhash,
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    SystemProgram,
    Transaction,
    TransactionInstruction,
} from "@solana/web3.js";
import { assert } from "chai";
import { before, describe, it } from "mocha";
import {
  type BanksClient,
  type ProgramTestContext,
  start,
} from "solana-bankrun";
  
// Constants
const PROGRAM_ID = new PublicKey(
  "H8ocBhDZmzxRvWnT1yu5EQyLN3D9AYZv9qsePcx8pidg",
);
const VAULT_SEED = Buffer.from("rent_vault");
const LOAD_LAMPORTS = LAMPORTS_PER_SOL; // 1 SOL

const instructionDiscriminators = {
    InitializeRentVault: Buffer.from([0]),
    DepositRent: Buffer.from([1]),
    CreateNewAccount: Buffer.from([2]),
}

describe("Pay the rent for an account using a PDA", () => {
  let context: ProgramTestContext;
  let lastBlock: Blockhash;
  let client: BanksClient;
  let payer: Keypair;
  
  const [vault_pda, _] = PublicKey.findProgramAddressSync(
    [VAULT_SEED],
    PROGRAM_ID,
  );

  before(async () => {
    context = await start(
      [{ name: "pda_rent_payer_program", programId: PROGRAM_ID }],
      [],
    );
    client = context.banksClient;
    payer = context.payer;
    lastBlock = context.lastBlockhash;
    
  });

  it("should initialize rent vault PDA", async () => {
    const data = Buffer.concat([instructionDiscriminators.InitializeRentVault]);
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: false },
        { pubkey: vault_pda, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data,
    });

    const tx = new Transaction();
    tx.recentBlockhash = lastBlock;
    tx.add(ix).sign(payer);

    // Process Transaction with all the instructions
    await client.processTransaction(tx);
  });

  it("should deposit rent into the vault", async () => {

  });

  it("should create new account using rent vault", async () => {
    const new_account = Keypair.generate();

    const data = Buffer.concat([instructionDiscriminators.CreateNewAccount]);

    const ix = new TransactionInstruction({
    keys: [
        { pubkey: vault_pda, isSigner: false, isWritable: true },
        { pubkey: new_account.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: PROGRAM_ID,
    data,
    });

    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix).sign(new_account);

    // Process Transaction with all the instructions
    const transaction = await client.processTransaction(tx);

    // assert(
    // transaction.logMessages[3].startsWith(
    //     "Program log: Created new account!",
    // ),
    // );
  });
});
