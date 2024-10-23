import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, clusterApiUrl } from '@solana/web3.js';
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
  const auth = new Keypair();

  const [authPDA, authBump] = PublicKey.findProgramAddressSync([Buffer.from('auth')], PROGRAM_ID);

  const [rentVaultPDA, vaultBump] = PublicKey.findProgramAddressSync([Buffer.from('vault'), auth.publicKey.toBuffer()], PROGRAM_ID);

  const [newAccountPDA, newAccountBump] = PublicKey.findProgramAddressSync([Buffer.from('new_account')], PROGRAM_ID);
  //   const [statePDA, stateBump] = PublicKey.findProgramAddressSync(
  //     [Buffer.from("state"), wallet.publicKey.toBuffer()],
  //     PROGRAM_ID
  //   );

  console.log(rentVaultPDA, 'this is the rent vault pda');
  console.log(authPDA, 'this is the auth pda');
  console.log(newAccountPDA, 'this is the new account pda');

  it('Initialize the Rent Vault', async () => {
    const tx = await program.methods
      .initRentVault()
      .accounts({
        owner: wallet.publicKey,
      })
      .rpc();

    // Check rent vault balance

    const accountInfo = await program.provider.connection.getAccountInfo(rentVaultPDA);

    console.log(accountInfo, 'this is the account info');

    const lamports = await connection.getMinimumBalanceForRentExemption(8);

    assert(accountInfo !== null, 'Rent vault account does not exist');

    assert(accountInfo.lamports === lamports, 'Incorrect rent vault balance');
  });

  it('Deposit to the Rent Vault', async () => {
    // 1 SOL
    const fundAmount = new anchor.BN(LAMPORTS_PER_SOL);
    await program.methods
      .depositToRentVault(fundAmount)
      .accounts({
        owner: wallet.publicKey,
      })
      .rpc();

    // Check rent vault balance
    const accountInfo = await program.provider.connection.getAccountInfo(rentVaultPDA);

    // 9 is the space the rentVault account occupies, poseidon automatically generates space for you when you initialize a pda
    const lamports = await connection.getMinimumBalanceForRentExemption(43);

    assert(accountInfo !== null, 'Rent vault account does not exist');

    assert(accountInfo.lamports === fundAmount.toNumber() + lamports, 'Incorrect rent vault balance');
  });

  it('Create a new account using the Rent Vault', async () => {
    const newAccount = new Keypair();

    const fundAmount = new anchor.BN(LAMPORTS_PER_SOL);

    await program.methods
      .createNewAccount(fundAmount)
      .accounts({
        owner: newAccount.publicKey,
      })
      .signers([newAccount])
      .rpc();

    // Check that the account was created
    const accountInfo = await connection.getAccountInfo(newAccountPDA);

    // 9 is the space the newAccount occupies, poseidon automatically generates space for you when you initialize a pda
    const lamports = await connection.getMinimumBalanceForRentExemption(41);

    console.log(lamports);
    console.log(accountInfo.lamports);

    assert(accountInfo !== null, 'Rent vault account does not exist');

    assert(accountInfo.lamports === fundAmount.toNumber() + lamports, 'Incorrect rent vault balance');
  });
});

//
//owner == payer, use payer

//payer --> rentvault --> newAccount
