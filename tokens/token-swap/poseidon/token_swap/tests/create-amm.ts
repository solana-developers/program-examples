import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { expect } from 'chai';
import { startAnchor } from 'solana-bankrun';
import type { TokenSwap } from '../target/types/token_swap';
import { type TestValues, createValues, expectRevert } from './utils';

const IDL = require('../target/idl/token_swap.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Create AMM', async () => {
  // Configure the client to use the anchor-bankrun
  const context = await startAnchor('', [{ name: 'token_swap', programId: PROGRAM_ID }], []);

  const provider = new BankrunProvider(context);

  const connection = provider.connection;

  const payer = provider.wallet as anchor.Wallet;

  const program = new anchor.Program<TokenSwap>(IDL, provider);

  let values: TestValues;

  beforeEach(() => {
    values = createValues();
  });

  it('Creation', async () => {
    const id = new anchor.BN(values.id);
    const fee = values.fee;
    await program.methods
      .createAmm(id, fee)
      .accounts({
        payer: payer.publicKey,
      })
      .rpc();

    const ammAccount = await program.account.amm.fetch(values.ammKey);
    expect(ammAccount.id.toString()).to.equal(values.id.toString());
    expect(ammAccount.admin.toString()).to.equal(values.admin.publicKey.toString());
    expect(ammAccount.fee.toString()).to.equal(values.fee.toString());
  });

  it('Invalid fee', async () => {
    const id = new anchor.BN(values.id);
    values.fee = 10000;

    await expectRevert(
      program.methods
        .createAmm(id, values.fee)
        .accounts({
          payer: payer.publicKey,
        })
        .rpc(),
    );
  });
});
