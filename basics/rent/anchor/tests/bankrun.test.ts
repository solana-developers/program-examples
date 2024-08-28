import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import Idl from '../target/idl/rent_example.json';
import type { RentExample } from '../target/types/rent_example';

const IDL = require('../target/idl/rent_example.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Bankrun example', async () => {
  const context = await startAnchor('', [{ name: 'rent_example', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);

  const wallet = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<RentExample>(IDL, provider);

  it('Create the account', async () => {
    const newKeypair = anchor.web3.Keypair.generate();

    const addressData: anchor.IdlTypes<RentExample>['addressData'] = {
      name: 'Marcus',
      address: '123 Main St. San Francisco, CA',
    };

    // We're just going to serialize our object here so we can check
    //  the size on the client side against the program logs
    //
    const addressDataBuffer = new anchor.BorshCoder(Idl as anchor.Idl).types.encode('AddressData', addressData);
    console.log(`Address data buffer length: ${addressDataBuffer.length}`);

    await program.methods
      .createSystemAccount(addressData)
      .accounts({
        payer: wallet.publicKey,
        newAccount: newKeypair.publicKey,
      })
      .signers([wallet.payer, newKeypair])
      .rpc();
  });
});
