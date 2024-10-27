import { type Blockhash, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { assert } from 'chai';
import { before, describe, it } from 'mocha';
import { type BanksClient, type ProgramTestContext, start } from 'solana-bankrun';

const PROGRAM_ID = new PublicKey('12rpZ18eGj7BeKvSFRZ45cni97HctTbKziBnW3MsH3NG');
const SEED = Buffer.from('createaccount'); // Convert to binary (bytes)

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

  it('Create the account via a cross program invocation', async () => {
    const [PDA] = await PublicKey.findProgramAddressSync([SEED], PROGRAM_ID);

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: PDA, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: Buffer.from([0]),
    });

    const tx = new Transaction();
    tx.recentBlockhash = lastBlock;
    tx.add(ix).sign(payer);

    // Process Transaction with all the instructions
    const transaction = await client.processTransaction(tx);

    assert(transaction.logMessages[3].startsWith('Program log: A new account has been created and initialized!'));
  });

  it('Create the account via direct call to system program', async () => {
    const newKeypair = Keypair.generate();

    const ix = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: newKeypair.publicKey,
      lamports: LAMPORTS_PER_SOL,
      space: 0,
      programId: SystemProgram.programId,
    });

    const tx = new Transaction();
    tx.recentBlockhash = lastBlock;
    tx.add(ix).sign(payer, newKeypair);

    await client.processTransaction(tx);
    console.log(`Account with public key ${newKeypair.publicKey} successfully created`);
  });
});
