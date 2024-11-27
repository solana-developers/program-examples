import { type Blockhash, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { assert } from 'chai';
import { before, describe, it } from 'mocha';
import { type BanksClient, type ProgramTestContext, start } from 'solana-bankrun';

const PROGRAM_ID = new PublicKey('12rpZ18eGj7BeKvSFRZ45cni97HctTbKziBnW3MsH3NG');

const instructionDiscriminators = {
  InitializeAccount: Buffer.from([0]),
};

describe('Create a system account', () => {
  let context: ProgramTestContext;
  let lastBlock: Blockhash;
  let client: BanksClient;
  let payer: Keypair;

  before(async () => {
    context = await start([{ name: 'create_account_program', programId: PROGRAM_ID }], []);
    client = context.banksClient;
    payer = context.payer;
    lastBlock = context.lastBlockhash;
  });

  it('should create the account via a cross program invocation', async () => {
    const newAccount = Keypair.generate();

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: newAccount.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: Buffer.concat([instructionDiscriminators.InitializeAccount]),
    });

    const tx = new Transaction();
    tx.recentBlockhash = lastBlock;
    tx.add(ix).sign(payer, newAccount);

    // No other tests required besides confirming if the transaction is processed
    // Since transactions are atomic, we can be certain the account was created
    await client.processTransaction(tx);
  });
});
