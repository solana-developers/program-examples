import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { RentProgram } from '../target/types/rent_program';

describe('rent_program', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);

  const program = anchor.workspace.RentProgram as Program<RentProgram>;

  const wallet = provider.wallet as anchor.Wallet;
  const addressData = {
    id: 87615,
    zipCode: 94016,
  };
  it('Create the account', async () => {
    const newKeypair = anchor.web3.Keypair.generate();

    await program.methods
      .createSystemAccount(new anchor.BN(addressData.id), new anchor.BN(addressData.zipCode))
      .accounts({
        owner: wallet.publicKey,
      })
      .signers([wallet.payer])
      .rpc();
  });
});
