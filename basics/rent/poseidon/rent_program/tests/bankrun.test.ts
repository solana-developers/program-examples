import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import type { RentProgram } from '../target/types/rent_program';

const IDL = require('../target/idl/rent_program.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Bankrun example', async () => {
  const context = await startAnchor('', [{ name: 'rent_program', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);

  const wallet = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<RentProgram>(IDL, provider);

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
