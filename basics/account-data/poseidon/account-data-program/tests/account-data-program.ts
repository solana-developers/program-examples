import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { assert } from 'chai';
import { AccountDataProgram } from '../target/types/account_data_program';

describe('account data program', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.AccountDataProgram as Program<AccountDataProgram>;

  const payer = provider.wallet as anchor.Wallet;

  it('Creates an address info account', async () => {
    console.log(`Payer: ${payer.publicKey}`);

    // Derive the Program Derived Address (PDA).
    const [pda, bump] = PublicKey.findProgramAddressSync([Buffer.from('address_info'), payer.publicKey.toBuffer()], program.programId);

    console.log(`PDA: ${pda}`);

    // Define the address info data.
    const addressInfo = {
      name: 'John Doe',
      houseNumber: 100,
      street: 62,
      cityCode: 47100,
    };

    // Call the create_address_info function on the program.
    await program.methods
      .createAddressInfo(addressInfo.houseNumber, addressInfo.street, addressInfo.cityCode, addressInfo.name)
      .accounts({
        payer: payer.publicKey,
      })
      .rpc();

    // Fetch the account data.
    const account = await program.account.addressInfo.fetch(pda);

    // Log and verify the initialized values.
    console.log(`Name: ${account.name}`);
    console.log(`House Number: ${account.houseNumber}`);
    console.log(`Street: ${account.street}`);
    console.log(`City Code: ${account.cityCode}`);

    // Assertions to verify account data.
    assert.strictEqual(account.name, addressInfo.name, 'Name does not match');
    assert.strictEqual(account.houseNumber, addressInfo.houseNumber, 'House Number does not match');
    assert.strictEqual(account.street, addressInfo.street, 'Street does not match');
    assert.strictEqual(account.cityCode, addressInfo.cityCode, 'City Code does not match');
  });
});
