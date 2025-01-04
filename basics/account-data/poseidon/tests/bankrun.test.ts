import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import type { AccountData } from '../target/types/account_data';

const IDL = require('../target/idl/account_data.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Account Data!', async () => {
  const context = await startAnchor('', [{ name: 'account_data', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);

  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<AccountData>(IDL, provider);

  // Generate a new keypair for the addressInfo account
  const [addressInfoAccount] = PublicKey.findProgramAddressSync([Buffer.from('address_info'), payer.publicKey.toBuffer()], program.programId);

  it('Create the address info account', async () => {
    console.log(`Payer Address      : ${payer.publicKey}`);
    console.log(`Address Info Acct  : ${addressInfoAccount}`);

    // Instruction Ix data
    const addressInfo = {
      name: 'Joe C',
      houseNumber: 136,
      street: 'Mile High Dr.',
      city: 'Solana Beach',
    };

    await program.methods
      .createAddressInfo(addressInfo.name, addressInfo.houseNumber, addressInfo.street, addressInfo.city)
      .accounts({
        addressInfo: addressInfoAccount,
        payer: payer.publicKey,
      })
      .rpc();
  });

  it("Read the new account's data", async () => {
    const addressInfo = await program.account.addressInfo.fetch(addressInfoAccount);
    console.log(`Name     : ${addressInfo.name}`);
    console.log(`House Num: ${addressInfo.houseNumber}`);
    console.log(`Street   : ${addressInfo.street}`);
    console.log(`City     : ${addressInfo.city}`);
  });
});
