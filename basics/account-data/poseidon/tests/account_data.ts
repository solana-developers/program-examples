import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { AddressInfo } from '../target/types/address_info';

describe('account data program', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.AddressInfo as Program<AddressInfo>;

  // Generate keypair for new account
  const addressInfoAccount = new Keypair();
  const [pda, bump] = PublicKey.findProgramAddressSync([Buffer.from('account_data'), addressInfoAccount.publicKey.toBuffer()], program.programId);

  it('Initialize address info', async () => {
    console.log(`Address Info Acct  : ${addressInfoAccount.publicKey}`);
    // Request an airdrop to fund the new account
    const airdropSignature = await provider.connection.requestAirdrop(
      addressInfoAccount.publicKey,
      1 * LAMPORTS_PER_SOL, // Request 1 SOL
    );
    await provider.connection.confirmTransaction(airdropSignature);

    // Instruction Ix data
    const addressInfo = {
      houseNumber: 100,
      streetNumber: 62,
      zipCode: 47100,
      countryCode: 1,
    };

    await program.methods
      .initialize(addressInfo.houseNumber, addressInfo.streetNumber, addressInfo.zipCode, addressInfo.countryCode)
      .accounts({
        owner: addressInfoAccount.publicKey,
      })
      .signers([addressInfoAccount])
      .rpc();

    const addressAccount = await program.account.addressInfoState.fetch(pda);
    console.log(`House Num: ${addressAccount.houseNumber}`);
    console.log(`Street Num: ${addressAccount.streetNumber}`);
    console.log(`Zip Code: ${addressAccount.zipCode}`);
    console.log(`Country Code: ${addressAccount.countryCode}`);
  });

  it('Edit address of account', async () => {
    // Instruction Ix data for edit
    const newAddressInfo = {
      houseNumber: 150,
      streetNumber: 80,
      zipCode: 48150,
      countryCode: 60,
    };

    await program.methods
      .edit(newAddressInfo.houseNumber, newAddressInfo.streetNumber, newAddressInfo.zipCode, newAddressInfo.countryCode)
      .accounts({
        owner: addressInfoAccount.publicKey,
      })
      .signers([addressInfoAccount])
      .rpc();

    const addressAccount = await program.account.addressInfoState.fetch(pda);
    console.log(`House Num: ${addressAccount.houseNumber}`);
    console.log(`Street Num: ${addressAccount.streetNumber}`);
    console.log(`Zip Code: ${addressAccount.zipCode}`);
    console.log(`Country Code: ${addressAccount.countryCode}`);
  });
});
