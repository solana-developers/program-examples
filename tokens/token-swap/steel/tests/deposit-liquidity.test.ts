import { beforeEach, describe, it } from 'node:test';
import { BankrunProvider } from 'anchor-bankrun';
import { expect } from 'chai';
import { AccountInfoBytes, start } from 'solana-bankrun';
import { createAmmTransactionInstruction, createDepositInstruction, createPoolInstruction } from './transactions';
import { TokenAccount, TokenLayout } from './types';
import { PROGRAM_ID, TestValues, createValues, mintingTokens } from './utils';

describe('Testing Deposit Liquidity', async () => {
  const context = await start([{ name: 'token_swap_program', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);
  const client = context.banksClient;
  const payer = context.payer;
  let values: TestValues;

  beforeEach(async () => {
    values = createValues();

    await mintingTokens({
      provider,
      creator: values.admin,
      context,
      mintAKeypair: values.mintAKeypair,
      mintBKeypair: values.mintBKeypair,
    });
    let tx = createAmmTransactionInstruction(values, payer, context);
    await client.processTransaction(tx);
    tx = createPoolInstruction(values, payer, context);
    await client.processTransaction(tx);
  });

  it('deposits equal amounts to the pool successfully', async () => {
    const tx = createDepositInstruction(values, payer, context, true);
    await client.processTransaction(tx);

    let depositTokenAccountLiquditiy: TokenAccount | AccountInfoBytes = await client.getAccount(values.liquidityAccount);
    depositTokenAccountLiquditiy = TokenLayout.decode(depositTokenAccountLiquditiy.data);
    expect(depositTokenAccountLiquditiy.amount.toString()).to.equal(values.depositAmountA.sub(values.minimumLiquidity).toString());

    let depositTokenAccountA: TokenAccount | AccountInfoBytes = await client.getAccount(values.holderAccountA);
    depositTokenAccountA = TokenLayout.decode(depositTokenAccountA.data);
    expect(depositTokenAccountA.amount.toString()).to.equal(values.defaultSupply.sub(values.depositAmountA).toString());

    let depositTokenAccountB: TokenAccount | AccountInfoBytes = await client.getAccount(values.holderAccountB);
    depositTokenAccountB = TokenLayout.decode(depositTokenAccountB.data);
    expect(depositTokenAccountB.amount.toString()).to.equal(values.defaultSupply.sub(values.depositAmountA).toString());
  });
});
