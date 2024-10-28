import { beforeEach, describe, it } from 'node:test';
import { BN } from '@coral-xyz/anchor';
import { BankrunProvider } from 'anchor-bankrun';
import { expect } from 'chai';
import { start } from 'solana-bankrun';
import { createAmmTransactionInstruction, createDepositInstruction, createPoolInstruction, createSwapInstruction } from './transactions';
import { TokenLayout } from './types';
import { PROGRAM_ID, TestValues, createValues, expectRevert, mintingTokens } from './utils';

describe('Testing Swap A for B', async () => {
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

    tx = createDepositInstruction(values, payer, context);
    await client.processTransaction(tx);
  });

  it('should swap token A for B successfully', async () => {
    const input = new BN(10 ** 6);
    const tx = createSwapInstruction(values, payer, context);
    await client.processTransaction(tx);

    const traderTokenAccountA = TokenLayout.decode((await client.getAccount(values.holderAccountA)).data);
    const traderTokenAccountB = TokenLayout.decode((await client.getAccount(values.holderAccountB)).data);

    expect(traderTokenAccountA.amount.toString()).to.equal(values.defaultSupply.sub(values.depositAmountA).sub(input).toString());
    expect(Number(traderTokenAccountB.amount.toString())).to.be.greaterThan(values.defaultSupply.sub(values.depositAmountB).toNumber());
    expect(Number(traderTokenAccountB.amount.toString())).to.be.lessThan(values.defaultSupply.sub(values.depositAmountB).add(input).toNumber());
  });

  it('should fail to swap if the output is smaller than the min_output_amount', async () => {
    const input = new BN(10 * 10 ** 6);
    const output = new BN(2 * 10 ** 6);
    const tx = createSwapInstruction(values, payer, context, input, output);
    const reverted = await expectRevert(client.processTransaction(tx));
    expect(reverted).to.equal(true, 'Expected Transaction to fail but it passed');
  });

  it('should not alter balances if swap amount is zero', async () => {
    const input = new BN(0); // Zero amount
    const tx = createSwapInstruction(values, payer, context, input, input);
    await client.processTransaction(tx);

    const traderTokenAccountA = TokenLayout.decode((await client.getAccount(values.holderAccountA)).data);
    const traderTokenAccountB = TokenLayout.decode((await client.getAccount(values.holderAccountB)).data);

    expect(traderTokenAccountA.amount.toString()).to.equal(values.defaultSupply.sub(values.depositAmountA).toString());
    expect(traderTokenAccountB.amount.toString()).to.equal(values.defaultSupply.sub(values.depositAmountB).toString());
  });

  it('should revert swap if pool setup is invalid', async () => {
    values = createValues();
    await mintingTokens({
      provider,
      creator: values.admin,
      context,
      mintAKeypair: values.mintAKeypair,
      mintBKeypair: values.mintBKeypair,
    });

    // Skip pool creation to simulate invalid setup
    const tx = createSwapInstruction(values, payer, context);

    const reverted = await expectRevert(client.processTransaction(tx));
    expect(reverted).to.equal(true, 'Expected Transaction to fail but it passed');
  });
});
