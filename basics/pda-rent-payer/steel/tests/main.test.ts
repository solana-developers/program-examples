import { type Blockhash, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { before, describe, it } from 'mocha';
import { type BanksClient, type ProgramTestContext, start } from 'solana-bankrun';

// Constants
const PROGRAM_ID = new PublicKey('HK5TuboXztZv7anSa3GptyCZ5wMYiqbY8kNSVEtqWDuD');
const VAULT_SEED = Buffer.from('rent_vault');
const LOAD_LAMPORTS = 1 * LAMPORTS_PER_SOL; // 1 SOL

const instructionDiscriminators = {
  InitializeRentVault: 0,
  CreateNewAccount: 1,
};

describe('Pay the rent for an account using a PDA', () => {
  // Helper classes and methods to serialize instruction data
  class Assignable {
    constructor(properties) {
      for (const [key, value] of Object.entries(properties)) {
        this[key] = value;
      }
    }
  }

  class InitializeRentVault extends Assignable {
    toBuffer() {
      return Buffer.from(borsh.serialize(InitRentVaultSchema, this));
    }
  }
  const InitRentVaultSchema = {
    struct: {
      instruction: 'u8',
      fund_lamports: 'u64',
    },
  };

  class CreateNewAccount extends Assignable {
    toBuffer() {
      return Buffer.from(borsh.serialize(CreateNewAccountSchema, this));
    }
  }
  const CreateNewAccountSchema = {
    struct: {
      instruction: 'u8',
    },
  };

  const [vault_pda, _] = PublicKey.findProgramAddressSync([VAULT_SEED], PROGRAM_ID);

  let context: ProgramTestContext;
  let client: BanksClient;
  let payer: Keypair;

  before(async () => {
    context = await start([{ name: 'pda_rent_payer_program', programId: PROGRAM_ID }], []);
    client = context.banksClient;
    payer = context.payer;
  });

  it('should initialize rent vault PDA', async () => {
    const ixdata = new InitializeRentVault({
      instruction: instructionDiscriminators.InitializeRentVault,
      fund_lamports: BigInt(LOAD_LAMPORTS),
    });
    const data = ixdata.toBuffer();

    const Createix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: false },
        { pubkey: vault_pda, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data,
    });

    // const Fundix = SystemProgram.transfer({
    //   fromPubkey: payer.publicKey,
    //   toPubkey: vault_pda,
    //   lamports: LOAD_LAMPORTS,
    // });

    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(Createix).sign(payer);

    // Process Transaction with all the instructions
    await client.processTransaction(tx);
  });

  it('should create new account using rent vault', async () => {
    const newAccount = Keypair.generate();

    const data = new CreateNewAccount({
      instruction: instructionDiscriminators.CreateNewAccount,
    }).toBuffer();

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: vault_pda, isSigner: false, isWritable: true },
        { pubkey: newAccount.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data,
    });

    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix).sign(newAccount);

    // Process Transaction with all the instructions
    await client.processTransaction(tx);

    // assert(
    // transaction.logMessages[3].startsWith(
    //     "Program log: Created new account!",
    // ),
    // );
  });
});
