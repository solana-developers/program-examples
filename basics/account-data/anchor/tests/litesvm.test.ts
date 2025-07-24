import { Program } from '@coral-xyz/anchor';
import { Keypair } from '@solana/web3.js';
import { LiteSVMProvider, fromWorkspace } from 'anchor-litesvm';
import { AnchorProgramExample } from '../target/types/anchor_program_example';
const IDL = require('../target/idl/anchor_program_example.json');

describe('anchor', () => {
  let client: any;
  let provider: LiteSVMProvider;
  let program: Program<AnchorProgramExample>;
  let payer: Keypair;
  let addressInfoAccount: Keypair;

  before(async () => {
    client = fromWorkspace('');
    provider = new LiteSVMProvider(client);
    payer = provider.wallet.payer;
    program = new Program<AnchorProgramExample>(IDL, provider);

    // a keypair for the addressInfo account
    addressInfoAccount = new Keypair();
  });

  it('Create the address info account', async () => {
    console.log(`Payer Address      : ${payer.publicKey}`);
    console.log(`Address Info Acct  : ${addressInfoAccount.publicKey}`);

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
        addressInfo: addressInfoAccount.publicKey,
        payer: payer.publicKey,
      })
      .signers([addressInfoAccount, payer])
      .rpc();
  });

  it("Read the new account's data", async () => {
    const addressInfo = await program.account.addressInfo.fetch(addressInfoAccount.publicKey);
    console.log(`Name     : ${addressInfo.name}`);
    console.log(`House Num: ${addressInfo.houseNumber}`);
    console.log(`Street   : ${addressInfo.street}`);
    console.log(`City     : ${addressInfo.city}`);
  });
});
