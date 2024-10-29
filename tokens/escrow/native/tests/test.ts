import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { Transaction } from '@solana/web3.js';
import { start } from 'solana-bankrun';
import { buildMakeOffer, buildTakeOffer } from './instruction';
import { createValues, mintingTokens } from './utils';

describe('Account Data!', async () => {
  const values = createValues();

  const context = await start([{ name: 'escrow_native_program', programId: values.programId }], []);

  const client = context.banksClient;
  const payer = context.payer;

  console.log(`Program Address    : ${values.programId}`);
  console.log(`Payer Address      : ${payer.publicKey}`);

  test('mint tokens to maker and taker', async () => {
    // mint token a to maker account
    await mintingTokens({
      context,
      holder: values.maker,
      mintKeypair: values.mintAKeypair,
    });

    // mint Token B to Taker account
    await mintingTokens({
      context,
      holder: values.taker,
      mintKeypair: values.mintBKeypair,
    });
  });

  test('Make Offer', async () => {
    const ix = buildMakeOffer({
      id: values.id,
      maker: values.maker.publicKey,
      maker_token_a: values.makerAccountA,
      offer: values.offer,
      token_a_offered_amount: values.amountA,
      token_b_wanted_amount: values.amountB,
      vault: values.vault,
      mint_a: values.mintAKeypair.publicKey,
      mint_b: values.mintBKeypair.publicKey,
      payer: payer.publicKey,
      programId: values.programId,
    });

    const blockhash = context.lastBlockhash;

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, values.maker);
    await client.processTransaction(tx);
  });

  test('Take Offer', async () => {
    const ix = buildTakeOffer({
      maker: values.maker.publicKey,
      offer: values.offer,
      vault: values.vault,
      mint_a: values.mintAKeypair.publicKey,
      mint_b: values.mintBKeypair.publicKey,
      maker_token_b: values.makerAccountB,
      taker: values.taker.publicKey,
      taker_token_a: values.takerAccountA,
      taker_token_b: values.takerAccountB,
      payer: payer.publicKey,
      programId: values.programId,
    });

    const blockhash = context.lastBlockhash;

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, values.taker);
    await client.processTransaction(tx);
  });
});
