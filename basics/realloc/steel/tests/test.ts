import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { start } from 'solana-bankrun';
import { AddressInfo, AddressInfoExtender, ExtendedAddressInfo, ReallocInstruction } from './schema';

describe('Realloc!', async () => {
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');
  const context = await start([{ name: 'realloc_steel_program', programId: PROGRAM_ID }], []);
  const client = context.banksClient;

  const addressInfoAccount = Keypair.generate();

  const payer = context.payer;

  test('Create the address info account', async () => {
    console.log(`Program Address    : ${PROGRAM_ID}`);
    console.log(`Payer Address      : ${payer.publicKey}`);
    console.log(`Address Info Acct  : ${addressInfoAccount.publicKey}`);

    const ixData = Buffer.concat([
      Buffer.from([ReallocInstruction.Create]),
      AddressInfo.fromData({
        name: 'Joe C',
        house_number: 136,
        street: 'Mile High Dr.',
        city: 'Solana Beach',
      }).toBuffer(),
    ]);

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        {
          pubkey: addressInfoAccount.publicKey,
          isSigner: true,
          isWritable: true,
        },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: ixData,
    });

    const blockhash = context.lastBlockhash;

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, addressInfoAccount);
    await client.processTransaction(tx);
  });

  test("Read the new account's data", async () => {
    const accountInfo = await client.getAccount(addressInfoAccount.publicKey);

    const readAddressInfo = AddressInfo.fromAccountData(Buffer.from(accountInfo.data)).toData();

    console.log(`Name     : ${readAddressInfo.name}`);
    console.log(`House Num: ${readAddressInfo.house_number}`);
    console.log(`Street   : ${readAddressInfo.street}`);
    console.log(`City     : ${readAddressInfo.city}`);
  });

  test('Extend the address info account', async () => {
    const ixData = Buffer.concat([
      Buffer.from([ReallocInstruction.Extend]),
      AddressInfoExtender.fromData({
        state: 'Illinois',
        zip: 12345,
      }).toBuffer(),
    ]);

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        {
          pubkey: addressInfoAccount.publicKey,
          isSigner: true,
          isWritable: true,
        },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: ixData,
    });

    const blockhash = context.lastBlockhash;

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, addressInfoAccount);
    await client.processTransaction(tx);
  });

  test("Read the new account's data", async () => {
    const accountInfo = await client.getAccount(addressInfoAccount.publicKey);

    const readAddressInfo = ExtendedAddressInfo.fromAccountData(Buffer.from(accountInfo.data)).toData();

    console.log(`Name     : ${readAddressInfo.name}`);
    console.log(`House Num: ${readAddressInfo.house_number}`);
    console.log(`Street   : ${readAddressInfo.street}`);
    console.log(`City     : ${readAddressInfo.city}`);
    console.log(`State    : ${readAddressInfo.state}`);
    console.log(`Zip      : ${readAddressInfo.zip}`);
  });
});
