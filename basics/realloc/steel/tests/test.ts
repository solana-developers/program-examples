import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { assert } from 'chai';
import { start } from 'solana-bankrun';
import { AddressInfo, AddressInfoExtender, ExtendedAddressInfo, ReallocInstruction, WorkInfo } from './schema';

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

    const addressInfo = {
      name: 'Joe C',
      house_number: 136,
      street: 'Mile High Dr.',
      city: 'Solana Beach',
    };

    const ixData = Buffer.concat([Buffer.from([ReallocInstruction.Create]), AddressInfo.fromData(addressInfo).toBuffer()]);

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

    const accountInfo = await client.getAccount(addressInfoAccount.publicKey);

    const readAddressInfo = AddressInfo.fromAccountData(Buffer.from(accountInfo.data)).toData();

    console.log(`Name     : ${readAddressInfo.name}`);
    console.log(`House Num: ${readAddressInfo.house_number}`);
    console.log(`Street   : ${readAddressInfo.street}`);
    console.log(`City     : ${readAddressInfo.city}`);

    assert(readAddressInfo.name.slice(0, addressInfo.name.length) === addressInfo.name, 'name does not match');
    assert(readAddressInfo.house_number === addressInfo.house_number, 'house number does not match');
    assert(readAddressInfo.street.slice(0, addressInfo.street.length) === addressInfo.street, 'street does not match');
    assert(readAddressInfo.city.slice(0, addressInfo.city.length) === addressInfo.city, 'city does not match');
  });

  test('Extend the address info account', async () => {
    const addressInfoExtender = {
      state: 'Illinois',
      zip: 12345,
    };

    const ixData = Buffer.concat([Buffer.from([ReallocInstruction.Extend]), AddressInfoExtender.fromData(addressInfoExtender).toBuffer()]);

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

    const accountInfo = await client.getAccount(addressInfoAccount.publicKey);

    const readAddressInfo = ExtendedAddressInfo.fromAccountData(Buffer.from(accountInfo.data)).toData();

    console.log(`Name     : ${readAddressInfo.name}`);
    console.log(`House Num: ${readAddressInfo.house_number}`);
    console.log(`Street   : ${readAddressInfo.street}`);
    console.log(`City     : ${readAddressInfo.city}`);
    console.log(`State    : ${readAddressInfo.state}`);
    console.log(`Zip      : ${readAddressInfo.zip}`);

    assert(readAddressInfo.state.slice(0, addressInfoExtender.state.length) === addressInfoExtender.state, 'state does not match');
    assert(readAddressInfo.zip === addressInfoExtender.zip, 'zip does not match');
  });

  test('zero init work info account', async () => {
    const workInfo = {
      name: 'Pete',
      company: 'Solana Labs',
      position: 'Engineer',
      years_employed: 2,
    };

    const ixData = Buffer.concat([Buffer.from([ReallocInstruction.ZeroInit]), WorkInfo.fromData(workInfo).toBuffer()]);

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

    const accountInfo = await client.getAccount(addressInfoAccount.publicKey);

    const readWorkInfo = WorkInfo.fromAccountData(Buffer.from(accountInfo.data)).toData();

    console.log(`Name          : ${readWorkInfo.name}`);
    console.log(`Position      : ${readWorkInfo.position}`);
    console.log(`Company       : ${readWorkInfo.company}`);
    console.log(`Years Employed: ${readWorkInfo.years_employed}`);

    assert(readWorkInfo.name.slice(0, workInfo.name.length) === workInfo.name, 'name does not match');
    assert(readWorkInfo.position.slice(0, workInfo.position.length) === workInfo.position, 'position does not match');
    assert(readWorkInfo.company.slice(0, workInfo.company.length) === workInfo.company, 'company does not match');
    assert(readWorkInfo.years_employed === workInfo.years_employed, 'years employed does not match');
  });
});
