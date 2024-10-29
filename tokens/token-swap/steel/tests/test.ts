import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { Transaction } from '@solana/web3.js';
import { BN } from 'bn.js';
import { start } from 'solana-bankrun';
import {
  buildCreateAmmInstruction,
  buildCreatePoolInstruction,
  buildDepositLiquidityInstruction,
  buildSwapExactTokensForTokensInstruction,
  buildWithdrawLiquidityInstruction,
} from './instruction';
import { createValues, mintingTokens } from './utils';

describe('Account Data!', async () => {
  const values = createValues();

  const context = await start([{ name: 'token_swap_steel_program', programId: values.programId }], []);

  const client = context.banksClient;
  const payer = context.payer;

  console.log(`Program Address    : ${values.programId}`);
  console.log(`Payer Address      : ${payer.publicKey}`);

  test('Create Amm', async () => {
    console.log('===================== CREATE AMM ======================');
    const ix = buildCreateAmmInstruction({
      admin: values.admin.publicKey,
      amm: values.ammKey,
      fee: values.fee,
      id: values.id,
      payer: payer.publicKey,
      programId: values.programId,
    });

    const blockhash = context.lastBlockhash;

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);
    await client.processTransaction(tx);
  });

  test(async () => {
    console.log('===================== Minting Tokens Before Tests ======================');
    await mintingTokens({
      context,
      creator: values.admin,
      mintAKeypair: values.mintAKeypair,
      mintBKeypair: values.mintBKeypair,
    });
  });

  test('Create Amm Pool', async () => {
    console.log('===================== CREATE AMM POOL ======================');
    const ix = buildCreatePoolInstruction({
      amm: values.ammKey,
      mintA: values.mintAKeypair.publicKey,
      mintB: values.mintBKeypair.publicKey,
      mintLiquidity: values.mintLiquidity,
      payer: payer.publicKey,
      pool: values.poolKey,
      poolAuthority: values.poolAuthority,
      poolTokenAccountA: values.poolAccountA,
      poolTokenAccountB: values.poolAccountB,
      programId: values.programId,
    });

    const blockhash = context.lastBlockhash;

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);
    await client.processTransaction(tx);
  });

  test('Deposit to amm pool', async () => {
    console.log('===================== DEPOSIT ======================');
    const ix = buildDepositLiquidityInstruction({
      amm: values.ammKey,
      mintA: values.mintAKeypair.publicKey,
      mintB: values.mintBKeypair.publicKey,
      mintLiquidity: values.mintLiquidity,
      payer: payer.publicKey,
      pool: values.poolKey,
      poolAuthority: values.poolAuthority,
      poolTokenAccountA: values.poolAccountA,
      poolTokenAccountB: values.poolAccountB,
      programId: values.programId,
      amount_a: values.depositAmountA,
      amount_b: values.depositAmountB,
      depositor: values.admin.publicKey,
      depositorTokenAccountA: values.holderAccountA,
      depositorTokenAccountB: values.holderAccountB,
      depositorTokenAccountLiquidity: values.liquidityAccount,
    });

    const blockhash = context.lastBlockhash;

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, values.admin);
    await client.processTransaction(tx);
  });

  test('Swap tokens for Tokens', async () => {
    console.log('===================== SWAP ======================');
    const ix = buildSwapExactTokensForTokensInstruction({
      amm: values.ammKey,
      mintA: values.mintAKeypair.publicKey,
      mintB: values.mintBKeypair.publicKey,
      mintLiquidity: values.mintLiquidity,
      payer: payer.publicKey,
      pool: values.poolKey,
      poolAuthority: values.poolAuthority,
      poolTokenAccountA: values.poolAccountA,
      poolTokenAccountB: values.poolAccountB,
      programId: values.programId,
      input_amount: new BN(10 ** 6),
      min_output_amount: new BN(100),
      swap_a: true,
      trader: values.admin.publicKey,
      traderTokenAccountA: values.holderAccountA,
      traderTokenAccountB: values.holderAccountB,
      traderTokenAccountLiquidity: values.liquidityAccount,
    });

    const blockhash = context.lastBlockhash;

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, values.admin);
    await client.processTransaction(tx);
  });

  test('Withdraw from amm pool', async () => {
    console.log('===================== WITHDRAW ======================');
    const ix = buildWithdrawLiquidityInstruction({
      amm: values.ammKey,
      mintA: values.mintAKeypair.publicKey,
      mintB: values.mintBKeypair.publicKey,
      mintLiquidity: values.mintLiquidity,
      payer: payer.publicKey,
      pool: values.poolKey,
      poolAuthority: values.poolAuthority,
      poolTokenAccountA: values.poolAccountA,
      poolTokenAccountB: values.poolAccountB,
      programId: values.programId,
      depositor: values.admin.publicKey,
      depositorTokenAccountA: values.holderAccountA,
      depositorTokenAccountB: values.holderAccountB,
      depositorTokenAccountLiquidity: values.liquidityAccount,
      amount: new BN(1000),
    });

    const blockhash = context.lastBlockhash;

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, values.admin);
    await client.processTransaction(tx);
  });
});
