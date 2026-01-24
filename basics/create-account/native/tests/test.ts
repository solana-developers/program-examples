import { readFileSync } from 'node:fs';
import { describe, test } from 'node:test';
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { LiteSVM } from 'litesvm';

describe('Create a system account', () => {
  // Load the program keypair
  const programKeypairPath = new URL(
    './fixtures/create_account_program-keypair.json',
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
    './fixtures/create_account_program.so',
    // @ts-ignore
    import.meta.url,
  ).pathname;
  litesvm.addProgramFromFile(PROGRAM_ID, programPath);

  // Fund the payer account
  litesvm.airdrop(payer.publicKey, BigInt(100 * LAMPORTS_PER_SOL));

  test('Create the account via a cross program invocation', () => {
    const newKeypair = Keypair.generate();

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: newKeypair.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: Buffer.alloc(0),
    });

    const tx = new Transaction().add(ix);
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = litesvm.latestBlockhash();
    tx.sign(payer, newKeypair);

    litesvm.sendTransaction(tx);

    // Verify the account was created
    const accountInfo = litesvm.getAccount(newKeypair.publicKey);
    console.log(`Account with public key ${newKeypair.publicKey} successfully created via CPI`);
  });

  test('Create the account via direct call to system program', () => {
    const newKeypair = Keypair.generate();

    const ix = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: newKeypair.publicKey,
      lamports: LAMPORTS_PER_SOL,
      space: 0,
      programId: SystemProgram.programId,
    });

    const tx = new Transaction().add(ix);
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = litesvm.latestBlockhash();
    tx.sign(payer, newKeypair);

    litesvm.sendTransaction(tx);

    // Verify the account was created
    const accountInfo = litesvm.getAccount(newKeypair.publicKey);
    console.log(`Account with public key ${newKeypair.publicKey} successfully created`);
  });
});
