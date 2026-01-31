import { describe, test } from 'node:test';
import { PublicKey, Transaction, LAMPORTS_PER_SOL, Keypair} from '@solana/web3.js';
import { LiteSVM } from 'litesvm';
import { createCloseUserInstruction, createCreateUserInstruction } from '../ts';

describe('Close Account!', async () => {
  const PROGRAM_ID = PublicKey.unique();
  const svm = new LiteSVM();
  svm.addProgramFromFile(PROGRAM_ID, 'tests/fixtures/close_account_native_program.so');
  
  const payer = Keypair.generate();
  svm.airdrop(payer.publicKey, BigInt(10 * LAMPORTS_PER_SOL));

  const testAccountPublicKey = PublicKey.findProgramAddressSync([Buffer.from('USER'), payer.publicKey.toBuffer()], PROGRAM_ID)[0];

  test('Create the account', async () => {
    const blockhash = svm.latestBlockhash();
    const ix = createCreateUserInstruction(testAccountPublicKey, payer.publicKey, PROGRAM_ID, 'Jacob');

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    svm.sendTransaction(tx);
  });

  test('Close the account', async () => {
    const blockhash = svm.latestBlockhash();

    const ix = createCloseUserInstruction(testAccountPublicKey, payer.publicKey, PROGRAM_ID);
    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    svm.sendTransaction(tx);
  });
});
