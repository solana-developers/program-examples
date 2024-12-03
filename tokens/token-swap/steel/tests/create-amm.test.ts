import { beforeEach, describe, it } from 'node:test';
import { expect } from 'chai';
import { AccountInfoBytes, start } from 'solana-bankrun';
import { createAmmTransactionInstruction } from './transactions';
import { Amm, AmmLayout } from './types';
import { PROGRAM_ID, TestValues, createValues, expectRevert } from './utils';

describe('Testing Creation of amm', async () => {
  const context = await start([{ name: 'token_swap_program', programId: PROGRAM_ID }], []);
  const client = context.banksClient;
  const payer = context.payer;
  let values: TestValues;

  beforeEach(() => {
    values = createValues();
  });

  it('creates an amm successfully', async () => {
    const tx = createAmmTransactionInstruction(values, payer, context);
    await client.processTransaction(tx);

    let ammAccount: Amm | AccountInfoBytes = await client.getAccount(values.ammKey);
    ammAccount = AmmLayout.decode(ammAccount.data);

    expect(ammAccount.id.toString()).to.equal(values.id.toString());
    expect(ammAccount.admin.toString()).to.equal(values.admin.publicKey.toString());
    expect(ammAccount.fee.toString()).to.equal(values.fee.toString());
  });

  it('failed to create an amm due to fee too high', async () => {
    const tx = createAmmTransactionInstruction(values, payer, context, true);
    const reverted = await expectRevert(client.processTransaction(tx));
    expect(reverted, 'This transaction should fail but it passed').to.be.true;
  });
});
