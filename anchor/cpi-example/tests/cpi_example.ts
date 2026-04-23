import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import { Keypair, PublicKey, SystemProgram, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { assert } from 'chai';
import type { CpiExample } from '../target/types/cpi_example';
import { 
  createMint, 
  createAccount, 
  mintTo, 
  getAccount,
  TOKEN_PROGRAM_ID 
} from '@solana/spl-token';

describe('cpi_example', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;

  const cpiExampleProgram = anchor.workspace.CpiExample as Program<CpiExample>;

  // Generate keypairs for accounts
  const cpiExampleKeypair = new Keypair();
  const mintKeypair = new Keypair();
  const fromTokenAccountKeypair = new Keypair();
  const toTokenAccountKeypair = new Keypair();
  const fromSolAccountKeypair = new Keypair();
  const toSolAccountKeypair = new Keypair();

  let mint: PublicKey;
  let fromTokenAccount: PublicKey;
  let toTokenAccount: PublicKey;

  it('Initialize the CPI example', async () => {
    // Initialize the CPI example program
    await cpiExampleProgram.methods
      .initialize()
      .accounts({
        cpiExample: cpiExampleKeypair.publicKey,
        authority: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([cpiExampleKeypair])
      .rpc();

    // Verify the CPI example was initialized correctly
    const cpiExampleAccount = await cpiExampleProgram.account.cpiExample.fetch(cpiExampleKeypair.publicKey);
    assert(cpiExampleAccount.authority.equals(payer.publicKey), 'Authority should be set correctly');
    assert(cpiExampleAccount.totalCpiCalls.toNumber() === 0, 'Total CPI calls should start at 0');
  });

  it('Create token mint and accounts for token CPI tests', async () => {
    // Create a new token mint
    mint = await createMint(
      provider.connection,
      payer.payer,
      payer.publicKey,
      null,
      9 // decimals
    );

    // Create token accounts
    fromTokenAccount = await createAccount(
      provider.connection,
      payer.payer,
      mint,
      payer.publicKey
    );

    toTokenAccount = await createAccount(
      provider.connection,
      payer.payer,
      mint,
      payer.publicKey
    );

    // Mint some tokens to the from account
    await mintTo(
      provider.connection,
      payer.payer,
      mint,
      fromTokenAccount,
      payer.publicKey,
      1000 * 10 ** 9 // 1000 tokens with 9 decimals
    );

    // Verify the token account has the correct balance
    const fromAccountInfo = await getAccount(provider.connection, fromTokenAccount);
    assert(fromAccountInfo.amount === BigInt(1000 * 10 ** 9), 'From account should have 1000 tokens');
  });

  it('Transfer tokens via CPI', async () => {
    const transferAmount = new anchor.BN(100 * 10 ** 9); // 100 tokens

    // Get initial balances
    const fromAccountBefore = await getAccount(provider.connection, fromTokenAccount);
    const toAccountBefore = await getAccount(provider.connection, toTokenAccount);

    // Call the transfer_tokens_via_cpi function
    await cpiExampleProgram.methods
      .transferTokensViaCpi(transferAmount)
      .accounts({
        cpiExample: cpiExampleKeypair.publicKey,
        fromTokenAccount: fromTokenAccount,
        toTokenAccount: toTokenAccount,
        authority: payer.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    // Verify the token transfer
    const fromAccountAfter = await getAccount(provider.connection, fromTokenAccount);
    const toAccountAfter = await getAccount(provider.connection, toTokenAccount);

    assert(
      fromAccountAfter.amount === fromAccountBefore.amount - BigInt(transferAmount.toString()),
      'From account should have 100 tokens less'
    );
    assert(
      toAccountAfter.amount === toAccountBefore.amount + BigInt(transferAmount.toString()),
      'To account should have 100 tokens more'
    );

    // Verify the CPI example state was updated
    const cpiExampleAccount = await cpiExampleProgram.account.cpiExample.fetch(cpiExampleKeypair.publicKey);
    assert(cpiExampleAccount.totalCpiCalls.toNumber() === 1, 'Total CPI calls should be 1');
  });

  it('Transfer SOL via CPI', async () => {
    const transferAmount = new anchor.BN(0.1 * LAMPORTS_PER_SOL); // 0.1 SOL

    // Get initial balances
    const fromAccountBefore = await provider.connection.getBalance(fromSolAccountKeypair.publicKey);
    const toAccountBefore = await provider.connection.getBalance(toSolAccountKeypair.publicKey);

    // Fund the from account
    const fundTx = await provider.connection.requestAirdrop(fromSolAccountKeypair.publicKey, 1 * LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(fundTx);

    // Call the transfer_sol_via_cpi function
    await cpiExampleProgram.methods
      .transferSolViaCpi(transferAmount)
      .accounts({
        cpiExample: cpiExampleKeypair.publicKey,
        fromAccount: fromSolAccountKeypair.publicKey,
        toAccount: toSolAccountKeypair.publicKey,
        authority: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([fromSolAccountKeypair])
      .rpc();

    // Verify the SOL transfer
    const fromAccountAfter = await provider.connection.getBalance(fromSolAccountKeypair.publicKey);
    const toAccountAfter = await provider.connection.getBalance(toSolAccountKeypair.publicKey);

    // Account for transaction fees
    const expectedFromBalance = fromAccountBefore + (1 * LAMPORTS_PER_SOL) - transferAmount.toNumber();
    const expectedToBalance = toAccountBefore + transferAmount.toNumber();

    assert(
      Math.abs(fromAccountAfter - expectedFromBalance) < 10000, // Allow for small fee differences
      'From account should have approximately the expected balance'
    );
    assert(
      toAccountAfter === expectedToBalance,
      'To account should have the exact expected balance'
    );

    // Verify the CPI example state was updated
    const cpiExampleAccount = await cpiExampleProgram.account.cpiExample.fetch(cpiExampleKeypair.publicKey);
    assert(cpiExampleAccount.totalCpiCalls.toNumber() === 2, 'Total CPI calls should be 2');
  });

  it('Multiple CPI calls in a single instruction', async () => {
    const tokenAmount = new anchor.BN(50 * 10 ** 9); // 50 tokens
    const solAmount = new anchor.BN(0.05 * LAMPORTS_PER_SOL); // 0.05 SOL

    // Get initial balances
    const fromTokenBefore = await getAccount(provider.connection, fromTokenAccount);
    const toTokenBefore = await getAccount(provider.connection, toTokenAccount);
    const fromSolBefore = await provider.connection.getBalance(fromSolAccountKeypair.publicKey);
    const toSolBefore = await provider.connection.getBalance(toSolAccountKeypair.publicKey);

    // Call the multiple_cpi_calls function
    await cpiExampleProgram.methods
      .multipleCpiCalls(tokenAmount, solAmount)
      .accounts({
        cpiExample: cpiExampleKeypair.publicKey,
        fromTokenAccount: fromTokenAccount,
        toTokenAccount: toTokenAccount,
        fromAccount: fromSolAccountKeypair.publicKey,
        toAccount: toSolAccountKeypair.publicKey,
        authority: payer.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([fromSolAccountKeypair])
      .rpc();

    // Verify the token transfer
    const fromTokenAfter = await getAccount(provider.connection, fromTokenAccount);
    const toTokenAfter = await getAccount(provider.connection, toTokenAccount);

    assert(
      fromTokenAfter.amount === fromTokenBefore.amount - BigInt(tokenAmount.toString()),
      'From token account should have 50 tokens less'
    );
    assert(
      toTokenAfter.amount === toTokenBefore.amount + BigInt(tokenAmount.toString()),
      'To token account should have 50 tokens more'
    );

    // Verify the SOL transfer
    const fromSolAfter = await provider.connection.getBalance(fromSolAccountKeypair.publicKey);
    const toSolAfter = await provider.connection.getBalance(toSolAccountKeypair.publicKey);

    const expectedFromSol = fromSolBefore - solAmount.toNumber();
    const expectedToSol = toSolBefore + solAmount.toNumber();

    assert(
      Math.abs(fromSolAfter - expectedFromSol) < 10000, // Allow for small fee differences
      'From SOL account should have approximately the expected balance'
    );
    assert(
      toSolAfter === expectedToSol,
      'To SOL account should have the exact expected balance'
    );

    // Verify the CPI example state was updated (should increment by 2 for 2 CPI calls)
    const cpiExampleAccount = await cpiExampleProgram.account.cpiExample.fetch(cpiExampleKeypair.publicKey);
    assert(cpiExampleAccount.totalCpiCalls.toNumber() === 4, 'Total CPI calls should be 4 (2 + 2)');
  });

  it('Transfer with PDA authority via CPI', async () => {
    const transferAmount = new anchor.BN(25 * 10 ** 9); // 25 tokens

    // Get initial balances
    const fromAccountBefore = await getAccount(provider.connection, fromTokenAccount);
    const toAccountBefore = await getAccount(provider.connection, toTokenAccount);

    // Find the PDA
    const [pdaAuthority] = PublicKey.findProgramAddressSync(
      [Buffer.from("cpi_example"), payer.publicKey.toBuffer()],
      cpiExampleProgram.programId
    );

    // Transfer authority of the from token account to the PDA
    const setAuthorityTx = await provider.connection.requestAirdrop(pdaAuthority, 0.1 * LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(setAuthorityTx);

    // Call the transfer_with_pda_authority function
    await cpiExampleProgram.methods
      .transferWithPdaAuthority(transferAmount)
      .accounts({
        cpiExample: cpiExampleKeypair.publicKey,
        fromTokenAccount: fromTokenAccount,
        toTokenAccount: toTokenAccount,
        pdaAuthority: pdaAuthority,
        authority: payer.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    // Verify the token transfer
    const fromAccountAfter = await getAccount(provider.connection, fromTokenAccount);
    const toAccountAfter = await getAccount(provider.connection, toTokenAccount);

    assert(
      fromAccountAfter.amount === fromAccountBefore.amount - BigInt(transferAmount.toString()),
      'From account should have 25 tokens less'
    );
    assert(
      toAccountAfter.amount === toAccountBefore.amount + BigInt(transferAmount.toString()),
      'To account should have 25 tokens more'
    );

    // Verify the CPI example state was updated
    const cpiExampleAccount = await cpiExampleProgram.account.cpiExample.fetch(cpiExampleKeypair.publicKey);
    assert(cpiExampleAccount.totalCpiCalls.toNumber() === 5, 'Total CPI calls should be 5');
  });

  it('Verify final state', async () => {
    // Verify final CPI example state
    const cpiExampleAccount = await cpiExampleProgram.account.cpiExample.fetch(cpiExampleKeypair.publicKey);
    assert(cpiExampleAccount.totalCpiCalls.toNumber() === 5, 'Final total CPI calls should be 5');

    // Verify final token balances
    const fromAccount = await getAccount(provider.connection, fromTokenAccount);
    const toAccount = await getAccount(provider.connection, toTokenAccount);
    
    // From account should have: 1000 - 100 - 50 - 25 = 825 tokens
    assert(fromAccount.amount === BigInt(825 * 10 ** 9), 'From account should have 825 tokens');
    // To account should have: 0 + 100 + 50 + 25 = 175 tokens
    assert(toAccount.amount === BigInt(175 * 10 ** 9), 'To account should have 175 tokens');
  });
});