import * as anchor from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import type { AccountData } from '../target/types/account_data';

describe('Account Data!', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.AccountData as anchor.Program<AccountData>;

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
