import { PublicKey, Keypair, SystemProgram, Transaction, TransactionInstruction, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { ProgramTestContext, BanksClient, start } from 'solana-bankrun';
import { createAMint, deserializeOfferAccount, encodeBigint, getMakeOfferInstructionData, getTakeOfferInstructionData, mintTo } from './utils';
import { AccountLayout, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { assert } from 'chai';

const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');

describe('Escrow Program', () => {
  let context: ProgramTestContext;
  let client: BanksClient;
  let payer: Keypair;
  let maker = Keypair.generate();
  let taker = Keypair.generate();

  const mint_a = Keypair.generate();
  const mint_b = Keypair.generate();

  let makerAccountA: PublicKey;
  let makerAccountB: PublicKey;
  let takerAccountA: PublicKey;
  let takerAccountB: PublicKey;
  const id = BigInt(1);
  const token_a_offered_amount = BigInt(2 * 10 ** 9);
  const token_b_wanted_amount = BigInt(5 * 10 ** 9);
  const [offer, offerBump] = PublicKey.findProgramAddressSync(
    [Buffer.from('offer'), maker.publicKey.toBuffer(), Buffer.from(encodeBigint(id))],
    PROGRAM_ID,
  );
  const vault = getAssociatedTokenAddressSync(mint_a.publicKey, offer, true);

  before(async () => {
    context = await start([{ name: 'escrow_program', programId: PROGRAM_ID }], []);
    client = context.banksClient;
    payer = context.payer;

    {
      const tx = new Transaction();
      tx.add(
        SystemProgram.transfer({
          fromPubkey: payer.publicKey,
          toPubkey: maker.publicKey,
          lamports: LAMPORTS_PER_SOL,
        }),
        SystemProgram.transfer({
          fromPubkey: payer.publicKey,
          toPubkey: taker.publicKey,
          lamports: LAMPORTS_PER_SOL,
        }),
      );
      tx.recentBlockhash = context.lastBlockhash;
      tx.sign(payer);

      await client.processTransaction(tx);
    }

    await createAMint(context, payer, mint_a);
    await createAMint(context, payer, mint_b);

    makerAccountA = getAssociatedTokenAddressSync(mint_a.publicKey, maker.publicKey, false);

    makerAccountB = getAssociatedTokenAddressSync(mint_b.publicKey, maker.publicKey, false);

    takerAccountA = getAssociatedTokenAddressSync(mint_a.publicKey, taker.publicKey, false);

    takerAccountB = getAssociatedTokenAddressSync(mint_b.publicKey, taker.publicKey, false);

    await mintTo(context, payer, maker.publicKey, mint_a.publicKey);
    // await mintTo(context, payer, maker.publicKey, mint_b.publicKey);
    // await mintTo(context, payer, taker.publicKey, mint_a.publicKey);
    await mintTo(context, payer, taker.publicKey, mint_b.publicKey);
  });

  it('Should make an offer successfully', async () => {
    const tx = new Transaction();
    tx.add(
      new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
          { pubkey: maker.publicKey, isSigner: true, isWritable: true },
          { pubkey: mint_a.publicKey, isSigner: false, isWritable: false },
          { pubkey: mint_b.publicKey, isSigner: false, isWritable: false },
          { pubkey: makerAccountA, isSigner: false, isWritable: true },
          { pubkey: offer, isSigner: false, isWritable: true },
          { pubkey: vault, isSigner: false, isWritable: true },
          {
            pubkey: TOKEN_PROGRAM_ID,
            isSigner: false,
            isWritable: false,
          },
          {
            pubkey: SystemProgram.programId,
            isSigner: false,
            isWritable: false,
          },
          {
            pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
            isSigner: false,
            isWritable: false,
          },
        ],
        data: getMakeOfferInstructionData(id, token_a_offered_amount, token_b_wanted_amount),
      }),
    );
    tx.recentBlockhash = context.lastBlockhash;
    tx.sign(maker);

    // process the transaction
    await client.processTransaction(tx);

    const offerAccount = await client.getAccount(offer);
    assert.isNotNull(offerAccount);
    assert.equal(offerAccount?.owner.toBase58(), PROGRAM_ID.toBase58());
    const offerAccountData = deserializeOfferAccount(offerAccount.data);
    assert.equal(offerAccountData.id, Number(id));
    assert.equal(offerAccountData.maker.toBase58(), maker.publicKey.toBase58());
    assert.equal(offerAccountData.token_mint_a.toBase58(), mint_a.publicKey.toBase58());
    assert.equal(offerAccountData.token_mint_b.toBase58(), mint_b.publicKey.toBase58());
    assert.equal(offerAccountData.token_b_wanted_amount, Number(token_b_wanted_amount));
    assert.equal(offerAccountData.bump, offerBump);

    const rawVaultAccount = await client.getAccount(vault);
    assert.isNotNull(rawVaultAccount);
    const decodedVaultAccount = AccountLayout.decode(rawVaultAccount?.data);
    assert.equal(decodedVaultAccount.amount, token_a_offered_amount);
  });

  it('Should take an offer successfully', async () => {
    const tx = new Transaction();
    tx.add(
      new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
          { pubkey: taker.publicKey, isSigner: true, isWritable: true },
          { pubkey: maker.publicKey, isSigner: false, isWritable: true },
          { pubkey: mint_a.publicKey, isSigner: false, isWritable: false },
          { pubkey: mint_b.publicKey, isSigner: false, isWritable: false },
          { pubkey: takerAccountA, isSigner: false, isWritable: true },
          { pubkey: takerAccountB, isSigner: false, isWritable: true },
          { pubkey: makerAccountB, isSigner: false, isWritable: true },
          { pubkey: offer, isSigner: false, isWritable: true },
          { pubkey: vault, isSigner: false, isWritable: true },
          {
            pubkey: TOKEN_PROGRAM_ID,
            isSigner: false,
            isWritable: false,
          },
          {
            pubkey: SystemProgram.programId,
            isSigner: false,
            isWritable: false,
          },
          {
            pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
            isSigner: false,
            isWritable: false,
          },
        ],
        data: getTakeOfferInstructionData(),
      }),
    );
    tx.recentBlockhash = context.lastBlockhash;
    tx.sign(taker);

    // process the transaction
    await client.processTransaction(tx);

    const rawMakerAccountB = await client.getAccount(makerAccountB);
    assert.isNotNull(rawMakerAccountB);
    const decodedMakerAccountB = AccountLayout.decode(rawMakerAccountB?.data);
    assert.equal(decodedMakerAccountB.amount, token_b_wanted_amount);

    const rawTakerAccountA = await client.getAccount(takerAccountA);
    assert.isNotNull(rawTakerAccountA);
    const decodedTakerAccountA = AccountLayout.decode(rawTakerAccountA?.data);
    assert.equal(decodedTakerAccountA.amount, token_a_offered_amount);

    const offerAccount = await client.getAccount(offer);
    assert.isNull(offerAccount);

    const vaultAccount = await client.getAccount(vault);
    assert.isNull(vaultAccount);
  });
});
