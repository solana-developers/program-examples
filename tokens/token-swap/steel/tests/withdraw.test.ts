import { beforeEach, describe, it } from 'node:test';
import { BN } from '@coral-xyz/anchor';
import { BankrunProvider } from 'anchor-bankrun';
import { expect } from 'chai';
import { AccountInfoBytes, start } from 'solana-bankrun';
import { createAmmTransactionInstruction, createDepositInstruction, createPoolInstruction, createWithdrawInstruction } from './transactions';
import { TokenAccount, TokenLayout } from './types';
import { PROGRAM_ID, TestValues, createValues, expectRevert, mintingTokens, sleep } from './utils';

describe('Testing Withdraw Liquidity', async () => {
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
    tx = createDepositInstruction(values, payer, context, true);
    await client.processTransaction(tx);
  });

  it('Withdraws everything successfully', async () => {
    const tx = createWithdrawInstruction(values, payer, context);
    await client.processTransaction(tx);

    let depositTokenAccountLiquditiy: TokenAccount | AccountInfoBytes = await client.getAccount(values.liquidityAccount);
    depositTokenAccountLiquditiy = TokenLayout.decode(depositTokenAccountLiquditiy.data);
    expect(depositTokenAccountLiquditiy.amount.toString()).to.equal('0');

    let depositTokenAccountA: TokenAccount | AccountInfoBytes = await client.getAccount(values.holderAccountA);
    depositTokenAccountA = TokenLayout.decode(depositTokenAccountA.data);
    expect(Number(depositTokenAccountA.amount)).to.be.lessThan(values.defaultSupply.toNumber());
    expect(Number(depositTokenAccountA.amount)).to.be.greaterThan(values.defaultSupply.sub(values.depositAmountA).toNumber());

    let depositTokenAccountB: TokenAccount | AccountInfoBytes = await client.getAccount(values.holderAccountB);
    depositTokenAccountB = TokenLayout.decode(depositTokenAccountB.data);

    expect(Number(depositTokenAccountB.amount)).to.be.lessThan(values.defaultSupply.toNumber());
    expect(Number(depositTokenAccountB.amount)).to.be.greaterThan(values.defaultSupply.sub(values.depositAmountA).toNumber());
  });

  it('should partially withdraw liquidity', async () => {
    const partialAmount = values.depositAmountA.div(new BN(2)); // 50% of the deposit amount
    const tx = createWithdrawInstruction(values, payer, context, partialAmount);
    await client.processTransaction(tx);

    const liquidityAccount = TokenLayout.decode((await client.getAccount(values.liquidityAccount)).data);

    expect(Number(liquidityAccount.amount)).to.be.greaterThan(0);
    expect(Number(liquidityAccount.amount)).to.be.lessThan(values.depositAmountA.toNumber());
  });

  it('should fail if attempting to withdraw more than available liquidity', async () => {
    const excessiveAmount = values.depositAmountA.add(new BN(10 ** 6)); // Beyond available liquidity
    const tx = createWithdrawInstruction(values, payer, context, excessiveAmount);

    const reverted = await expectRevert(client.processTransaction(tx));
    expect(reverted).to.equal(true, 'Expected Transaction to fail but it passed');
  });

  it('should fail to withdraw if pool setup is invalid', async () => {
    values = createValues(); // New setup without pool creation

    await mintingTokens({
      provider,
      creator: values.admin,
      context,
      mintAKeypair: values.mintAKeypair,
      mintBKeypair: values.mintBKeypair,
    });

    const tx = createWithdrawInstruction(values, payer, context, values.depositAmountA);
    const reverted = await expectRevert(client.processTransaction(tx));
    expect(reverted).to.equal(true, 'Expected Transaction to fail but it passed');
  });
});
