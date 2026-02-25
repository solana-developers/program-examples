import { describe, test } from 'node:test';
import { AccountLayout } from '@solana/spl-token';
import { Keypair, SystemProgram, Transaction } from '@solana/web3.js';
import { assert } from 'chai';
import { start } from 'solana-bankrun';
import { OfferAccount } from './account';
import { buildMakeOffer, buildTakeOffer } from './instruction';
import { createValues, mintingTokens } from './utils';

describe('Escrow!', async () => {
  const values = createValues();

  const context = await start([{ name: 'escrow_native_program', programId: values.programId }], []);

  const client = context.banksClient;
  const payer = context.payer;

  // Used to reproduce the "rent refund to arbitrary payer" bug: provide a
  // different signer as `payer` for TakeOffer.
  const attacker = Keypair.generate();
  let offerRentLamports = 0n;

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

    const offerInfo = await client.getAccount(values.offer);
    offerRentLamports = BigInt(offerInfo.lamports);
    const offer = OfferAccount.fromBuffer(offerInfo.data).toData();

    const vaultInfo = await client.getAccount(values.vault);
    const vaultTokenAccount = AccountLayout.decode(vaultInfo.data);

    assert(offer.id.toString() === values.id.toString(), 'wrong id');
    assert(offer.maker.toBase58() === values.maker.publicKey.toBase58(), 'maker key does not match');
    assert(offer.token_mint_a.toBase58() === values.mintAKeypair.publicKey.toBase58(), 'wrong mint A');
    assert(offer.token_mint_b.toBase58() === values.mintBKeypair.publicKey.toBase58(), 'wrong mint B');
    assert(offer.token_b_wanted_amount.toString() === values.amountB.toString(), 'unexpected amount B');
    assert(vaultTokenAccount.amount.toString() === values.amountA.toString(), 'unexpected amount A');
  });

  test('Take Offer (rent refunded to maker, not arbitrary payer)', async () => {
    // Ensure system accounts exist with known starting balances.
    // (Bankrun doesn't materialize system accounts until they hold lamports.)
    const fundTx = new Transaction();
    fundTx.recentBlockhash = context.lastBlockhash;
    fundTx.feePayer = payer.publicKey;
    fundTx
      .add(
        SystemProgram.transfer({
          fromPubkey: payer.publicKey,
          toPubkey: attacker.publicKey,
          lamports: 5_000_000,
        }),
      )
      .add(
        SystemProgram.transfer({
          fromPubkey: payer.publicKey,
          toPubkey: values.maker.publicKey,
          lamports: 5_000_000,
        }),
      )
      .sign(payer);
    await client.processTransaction(fundTx);

    const makerBefore = BigInt((await client.getAccount(values.maker.publicKey)).lamports);
    const attackerBefore = BigInt((await client.getAccount(attacker.publicKey)).lamports);

    // Build TakeOffer with an arbitrary attacker-controlled payer.
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
      payer: attacker.publicKey,
      programId: values.programId,
    });

    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.feePayer = payer.publicKey; // keep fees deterministic and away from attacker
    tx.add(ix).sign(payer, attacker, values.taker);
    await client.processTransaction(tx);

    const makerAfter = BigInt((await client.getAccount(values.maker.publicKey)).lamports);
    const attackerAfter = BigInt((await client.getAccount(attacker.publicKey)).lamports);

    // Security property: offer account rent must not be transferable to an arbitrary `payer`.
    assert(makerAfter - makerBefore === offerRentLamports, 'maker did not receive offer rent');
    assert(attackerAfter <= attackerBefore, 'attacker unexpectedly received offer rent');

    const offerInfo = await client.getAccount(values.offer);
    assert(offerInfo === null, 'offer account not closed');

    const vaultInfo = await client.getAccount(values.vault);
    assert(vaultInfo === null, 'vault account not closed');

    const makerTokenBInfo = await client.getAccount(values.makerAccountB);
    const makerTokenAccountB = AccountLayout.decode(makerTokenBInfo.data);

    const takerTokenAInfo = await client.getAccount(values.takerAccountA);
    const takerTokenAccountA = AccountLayout.decode(takerTokenAInfo.data);

    assert(takerTokenAccountA.amount.toString() === values.amountA.toString(), 'unexpected amount a');
    assert(makerTokenAccountB.amount.toString() === values.amountB.toString(), 'unexpected amount b');
  });
});
