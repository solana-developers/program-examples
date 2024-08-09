import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { startAnchor } from 'solana-bankrun';
import type { PdaRentPayer } from '../target/types/pda_rent_payer';

const IDL = require('../target/idl/pda_rent_payer.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('PDA Rent-Payer', async () => {
  const context = await startAnchor('', [{ name: 'pda_rent_payer', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);
  const program = new anchor.Program<PdaRentPayer>(IDL, provider);

  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  // PDA for the Rent Vault
  const [rentVaultPDA] = PublicKey.findProgramAddressSync([Buffer.from('rent_vault')], program.programId);

  it('Initialize the Rent Vault', async () => {
    // 1 SOL
    const fundAmount = new anchor.BN(LAMPORTS_PER_SOL);

    await program.methods
      .initRentVault(fundAmount)
      .accounts({
        payer: wallet.publicKey,
      })
      .rpc();

    // Check rent vault balance
    const accountInfo = await program.provider.connection.getAccountInfo(rentVaultPDA);
    assert(accountInfo.lamports === fundAmount.toNumber());
  });

  it('Create a new account using the Rent Vault', async () => {
    // Generate a new keypair for the new account
    const newAccount = new Keypair();

    await program.methods
      .createNewAccount()
      .accounts({
        newAccount: newAccount.publicKey,
      })
      .signers([newAccount])
      .rpc();

    // Minimum balance for rent exemption for new account
    const lamports = await connection.getMinimumBalanceForRentExemption(0);

    // Check that the account was created
    const accountInfo = await connection.getAccountInfo(newAccount.publicKey);
    assert(accountInfo.lamports === lamports);
  });
});
