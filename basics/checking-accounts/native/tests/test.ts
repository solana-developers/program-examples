import { readFileSync } from 'node:fs';
import { describe, test } from 'node:test';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { LiteSVM } from 'litesvm';

describe('Checking accounts', () => {
  // Load the program keypair
  const programKeypairPath = new URL(
    './fixtures/checking_accounts_native_program-keypair.json',
    // @ts-ignore
    import.meta.url,
  ).pathname;
  const programKeypairData = JSON.parse(readFileSync(programKeypairPath, 'utf-8'));
  const programKeypair = Keypair.fromSecretKey(new Uint8Array(programKeypairData));
  const PROGRAM_ID = programKeypair.publicKey;

  const litesvm = new LiteSVM();
  const payer = Keypair.generate();

  // Load the program
  const programPath = new URL(
    './fixtures/checking_accounts_native_program.so',
    // @ts-ignore
    import.meta.url,
  ).pathname;
  litesvm.addProgramFromFile(PROGRAM_ID, programPath);

  // Fund the payer account
  litesvm.airdrop(payer.publicKey, BigInt(100000000000));

  // We'll create this ahead of time.
  // Our program will try to modify it.
  const accountToChange = Keypair.generate();
  // Our program will create this.
  const accountToCreate = Keypair.generate();

  test('Create an account owned by our program', () => {
    const ix = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: accountToChange.publicKey,
      lamports: 0, // Minimum rent for 0 space
      space: 0,
      programId: PROGRAM_ID, // Our program
    });

    const tx = new Transaction().add(ix);
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = litesvm.latestBlockhash();
    tx.sign(payer, accountToChange);

    litesvm.sendTransaction(tx);
  });

  test('Check accounts', () => {
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

    const tx = new Transaction().add(ix);
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = litesvm.latestBlockhash();
    tx.sign(payer, accountToChange, accountToCreate);

    litesvm.sendTransaction(tx);
  });
});
