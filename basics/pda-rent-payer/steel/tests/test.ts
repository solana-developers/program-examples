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
  "12rpZ18eGj7BeKvSFRZ45cni97HctTbKziBnW3MsH3NG",
);
const VAULT_SEED = Buffer.from("rent_vault");
const NEW_ACCOUNT_SEED = Buffer.from("new_account");
const LOAD_LAMPORTS = LAMPORTS_PER_SOL; // 1 SOL

const instructionDiscriminators = {
    InitializeRentVault: Buffer.from([0]),
    CreateNewAccount: Buffer.from([1]),
}

describe("Pay the rent for an account using a PDA", () => {
  let context: ProgramTestContext;
  let lastBlock: Blockhash;
  let client: BanksClient;
  let payer: Keypair;

  before(async () => {
    context = await start(
      [{ name: "pda_rent_payer_program", programId: PROGRAM_ID }],
      [],
    );
    client = context.banksClient;
    payer = context.payer;
    lastBlock = context.lastBlockhash;
  });

  it("Initialize rent vault PDA", async () => {
    const [vault_pda, _] = await PublicKey.findProgramAddressSync(
      [VAULT_SEED, payer.publicKey.toBuffer()],
      PROGRAM_ID,
    );

    const data = Buffer.concat([instructionDiscriminators.InitializeRentVault, Buffer.from([LOAD_LAMPORTS])]);

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: vault_pda, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data,
    });

    const tx = new Transaction();
    tx.recentBlockhash = lastBlock;
    tx.add(ix).sign(payer);

    // Process Transaction with all the instructions
    const transaction = await client.processTransaction(tx);

    assert(
      transaction.logMessages[3].startsWith(
        "Program log: Initialized rent vault.",
      ),
    );
  });

  it("Create new account using rent vault", async () => {
    const new_account = Keypair.generate();

    const data = instructionDiscriminators.CreateNewAccount;

    const ix = new TransactionInstruction({
    keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: new_account.publicKey, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: PROGRAM_ID,
    data,
    });

    const tx = new Transaction();
    tx.recentBlockhash = lastBlock;
    tx.add(ix).sign(payer, new_account);

    // Process Transaction with all the instructions
    const transaction = await client.processTransaction(tx);

    assert(
    transaction.logMessages[3].startsWith(
        "Program log: Created new account!",
    ),
    );
  });
});
