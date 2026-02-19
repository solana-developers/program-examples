import { describe, test } from 'node:test';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction, LAMPORTS_PER_SOL} from '@solana/web3.js';
import { LiteSVM } from 'litesvm';

describe('Checking accounts', async () => {
  const PROGRAM_ID = PublicKey.unique();
  const svm = new LiteSVM();
  svm.addProgramFromFile(PROGRAM_ID, 'tests/fixtures/checking_accounts_native_program.so');
  
  const payer = Keypair.generate();
  svm.airdrop(payer.publicKey, BigInt(10 * LAMPORTS_PER_SOL));
  const rent = svm.getRent();

  // We'll create this ahead of time.
  // Our program will try to modify it.
  const accountToChange = Keypair.generate();
  // Our program will create this.
  const accountToCreate = Keypair.generate();

  test('Create an account owned by our program', async () => {
    const blockhash = svm.latestBlockhash();
    const ix = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: accountToChange.publicKey,
      lamports: Number(rent.minimumBalance(BigInt(0))),
      space: 0,
      programId: PROGRAM_ID, // Our program
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, accountToChange);

    svm.sendTransaction(tx);
  });

  test('Check accounts', async () => {
    const blockhash = svm.latestBlockhash();
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: accountToCreate.publicKey, isSigner: true, isWritable: true },
        { pubkey: accountToChange.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: Buffer.alloc(0),
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, accountToChange, accountToCreate);

    svm.sendTransaction(tx);
  });
});
