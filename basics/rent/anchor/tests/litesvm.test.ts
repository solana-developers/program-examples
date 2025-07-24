import * as anchor from '@coral-xyz/anchor';
import { LiteSVMProvider, fromWorkspace } from 'anchor-litesvm';
import type { RentExample } from '../target/types/rent_example';

const IDL = require('../target/idl/rent_example.json');

describe('rent example', () => {
  // Configure the Anchor provider & load the program IDL for LiteSVM
  // The IDL gives you a typescript module
  const client = fromWorkspace('');
  const provider = new LiteSVMProvider(client);
  const payer = provider.wallet.payer;
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
    const addressDataBuffer = new anchor.BorshCoder(IDL as anchor.Idl).types.encode('AddressData', addressData);
    console.log(`Address data buffer length: ${addressDataBuffer.length}`);

    await program.methods
      .createSystemAccount(addressData)
      .accounts({
        payer: payer.publicKey,
        newAccount: newKeypair.publicKey,
      })
      .signers([payer, newKeypair])
      .rpc();
  });
});
