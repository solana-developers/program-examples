import { describe, test } from 'node:test';
import { Keypair, PublicKey, Transaction, LAMPORTS_PER_SOL} from '@solana/web3.js';
import { LiteSVM } from 'litesvm';
import {
  AddressInfo,
  createCreateInstruction,
  createReallocateWithoutZeroInitInstruction,
  createReallocateZeroInitInstruction,
  EnhancedAddressInfo,
  WorkInfo,
} from '../ts';

describe('Realloc!', async () => {
  const PROGRAM_ID = PublicKey.unique();
  const svm = new LiteSVM();
  svm.addProgramFromFile(PROGRAM_ID, 'tests/fixtures/realloc_program.so');
  
  const payer = Keypair.generate();
  svm.airdrop(payer.publicKey, BigInt(10 * LAMPORTS_PER_SOL));

  const testAccount = Keypair.generate();

  test('Create the account with data', () => {
    console.log(`${testAccount.publicKey}`);
    const ix = createCreateInstruction(testAccount.publicKey, payer.publicKey, PROGRAM_ID, 'Jacob', 123, 'Main St.', 'Chicago');

    const tx = new Transaction();
    tx.recentBlockhash = svm.latestBlockhash();
    tx.add(ix).sign(payer, testAccount);
    svm.sendTransaction(tx);

    printAddressInfo(testAccount.publicKey);
  });

  test('Reallocate WITHOUT zero init', () => {
    const ix = createReallocateWithoutZeroInitInstruction(testAccount.publicKey, payer.publicKey, PROGRAM_ID, 'Illinois', 12345);
    const tx = new Transaction();
    svm.expireBlockhash();
    tx.recentBlockhash = svm.latestBlockhash();
    tx.add(ix).sign(payer);
    svm.sendTransaction(tx);

    printEnhancedAddressInfo(testAccount.publicKey);
  });

  test('Reallocate WITH zero init', () => {
    const ix = createReallocateZeroInitInstruction(testAccount.publicKey, payer.publicKey, PROGRAM_ID, 'Pete', 'Engineer', 'Solana Labs', 2);
    const tx = new Transaction();
    svm.expireBlockhash();
    tx.recentBlockhash = svm.latestBlockhash();
    tx.add(ix).sign(payer);
    svm.sendTransaction(tx);

    printEnhancedAddressInfo(testAccount.publicKey);
    printWorkInfo(testAccount.publicKey);
  });

  function printAddressInfo(pubkey: PublicKey): void {
    const data = (svm.getAccount(pubkey))?.data;
    if (data) {
      const addressInfo = AddressInfo.fromBuffer(Buffer.from(data));
      console.log('Address info:');
      console.log(`   Name:       ${addressInfo.name}`);
      console.log(`   House Num:  ${addressInfo.house_number}`);
      console.log(`   Street:     ${addressInfo.street}`);
      console.log(`   City:       ${addressInfo.city}`);
    }
  }

  function printEnhancedAddressInfo(pubkey: PublicKey): void {
    const data = (svm.getAccount(pubkey))?.data;
    if (data) {
      const enhancedAddressInfo = EnhancedAddressInfo.fromBuffer(Buffer.from(data));
      console.log('Enhanced Address info:');
      console.log(`   Name:       ${enhancedAddressInfo.name}`);
      console.log(`   House Num:  ${enhancedAddressInfo.house_number}`);
      console.log(`   Street:     ${enhancedAddressInfo.street}`);
      console.log(`   City:       ${enhancedAddressInfo.city}`);
      console.log(`   State:      ${enhancedAddressInfo.state}`);
      console.log(`   Zip:        ${enhancedAddressInfo.zip}`);
    }
  }

  function printWorkInfo(pubkey: PublicKey): void {
    const data = (svm.getAccount(pubkey))?.data;
    if (data) {
      const workInfo = WorkInfo.fromBuffer(Buffer.from(data));
      console.log('Work info:');
      console.log(`   Name:       ${workInfo.name}`);
      console.log(`   Position:   ${workInfo.position}`);
      console.log(`   Company:    ${workInfo.company}`);
      console.log(`   Years:      ${workInfo.years_employed}`);
    }
  }
});
