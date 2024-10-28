import { Connection, Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Transaction, TransactionInstruction } from '@solana/web3.js';
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID, unpackAccount } from '@solana/spl-token';
import { assert } from 'chai';
import { describe, it } from 'mocha';
import { BanksClient, ProgramTestContext, start } from 'solana-bankrun';
import {
  createAMint,
  deserializeAmmAccount,
  deserializePoolAccount,
  getCreateAmmInstructionData,
  getCreatePoolInstructionData,
  getDepositLiquidityInstructionData,
  getWithdrawLiquidityInstructionData,
  mintTo,
} from './utils';

const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');

describe('Account Data Program', () => {
  let context: ProgramTestContext;
  let client: BanksClient;
  let payer: Keypair;
  const mint_a = Keypair.generate();
  const mint_b = Keypair.generate();
  const admin = Keypair.generate();
  const fee = 1000; // 10%

  const id = Keypair.generate();
  const [ammPda] = PublicKey.findProgramAddressSync([id.publicKey.toBuffer()], PROGRAM_ID);

  const [poolPda] = PublicKey.findProgramAddressSync([ammPda.toBuffer(), mint_a.publicKey.toBuffer(), mint_b.publicKey.toBuffer()], PROGRAM_ID);

  const [poolAuthorityPda] = PublicKey.findProgramAddressSync(
    [ammPda.toBuffer(), mint_a.publicKey.toBuffer(), mint_b.publicKey.toBuffer(), Buffer.from('authority')],
    PROGRAM_ID,
  );

  const [mintLiquidityPda] = PublicKey.findProgramAddressSync(
    [ammPda.toBuffer(), mint_a.publicKey.toBuffer(), mint_b.publicKey.toBuffer(), Buffer.from('liquidity')],
    PROGRAM_ID,
  );

  const poolAccountA = getAssociatedTokenAddressSync(mint_a.publicKey, poolAuthorityPda, true);

  const poolAccountB = getAssociatedTokenAddressSync(mint_b.publicKey, poolAuthorityPda, true);

  let depositorAccountLp: PublicKey;
  let depositorAccountA: PublicKey;
  let depositorAccountB: PublicKey;

  const amountA = BigInt(4 * 10 ** 9);
  const amountB = BigInt(1 * 10 ** 9);
  const amountLp = BigInt(2 * 10 ** 9) - BigInt(100);

  before(async () => {
    context = await start([{ name: 'token_swap_program', programId: PROGRAM_ID }], []);
    client = context.banksClient;
    payer = context.payer;
    console.log(mint_a.publicKey.toBase58(), payer.publicKey.toBase58());
    await createAMint(context, payer, mint_a);
    await createAMint(context, payer, mint_b);

    depositorAccountLp = getAssociatedTokenAddressSync(mintLiquidityPda, payer.publicKey, false);

    depositorAccountA = getAssociatedTokenAddressSync(mint_a.publicKey, payer.publicKey, false);

    depositorAccountB = getAssociatedTokenAddressSync(mint_b.publicKey, payer.publicKey, false);

    await mintTo(context, payer, payer.publicKey, mint_a.publicKey);
    await mintTo(context, payer, payer.publicKey, mint_b.publicKey);
  });

  it('Should create a new amm successfully', async () => {
    const tx = new Transaction();
    tx.add(
      new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: admin.publicKey, isSigner: false, isWritable: false },
          { pubkey: ammPda, isSigner: false, isWritable: true },
          {
            pubkey: SystemProgram.programId,
            isSigner: false,
            isWritable: false,
          },
        ],
        data: getCreateAmmInstructionData(id.publicKey, fee),
      }),
    );
    tx.recentBlockhash = context.lastBlockhash;
    tx.sign(payer);

    // process the transaction
    await client.processTransaction(tx);

    const ammAccount = await client.getAccount(ammPda);
    assert.isNotNull(ammAccount);
    assert.equal(ammAccount?.owner.toBase58(), PROGRAM_ID.toBase58());
    const ammAccountData = deserializeAmmAccount(ammAccount.data);

    assert.equal(ammAccountData.id.toBase58(), id.publicKey.toBase58());
    assert.equal(ammAccountData.admin.toBase58(), admin.publicKey.toBase58());
    assert.equal(ammAccountData.fee, fee);
  });

  it('Should create a new pool successfully', async () => {
    const tx = new Transaction();
    tx.add(
      new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: ammPda, isSigner: false, isWritable: false },
          { pubkey: poolPda, isSigner: false, isWritable: true },
          { pubkey: poolAuthorityPda, isSigner: false, isWritable: false },
          { pubkey: mintLiquidityPda, isSigner: false, isWritable: true },
          { pubkey: mint_a.publicKey, isSigner: false, isWritable: false },
          { pubkey: mint_b.publicKey, isSigner: false, isWritable: false },
          { pubkey: poolAccountA, isSigner: false, isWritable: true },
          { pubkey: poolAccountB, isSigner: false, isWritable: true },
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
          {
            pubkey: SYSVAR_RENT_PUBKEY,
            isSigner: false,
            isWritable: false,
          },
        ],
        data: getCreatePoolInstructionData(),
      }),
    );
    tx.recentBlockhash = context.lastBlockhash;
    tx.sign(payer);

    // process the transaction
    await client.processTransaction(tx);

    const poolPdaAccount = await client.getAccount(poolPda);
    assert.isNotNull(poolPdaAccount);
    assert.equal(poolPdaAccount?.owner.toBase58(), PROGRAM_ID.toBase58());

    const data = deserializePoolAccount(poolPdaAccount.data);
    assert.equal(data.amm.toBase58(), ammPda.toBase58());
    assert.equal(data.mintA.toBase58(), mint_a.publicKey.toBase58());
    assert.equal(data.mintB.toBase58(), mint_b.publicKey.toBase58());
  });

  it('Should deposit liquidity successfully', async () => {
    const tx = new Transaction();
    tx.add(
      new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: poolPda, isSigner: false, isWritable: true },
          { pubkey: poolAuthorityPda, isSigner: false, isWritable: false },
          { pubkey: mintLiquidityPda, isSigner: false, isWritable: true },
          { pubkey: mint_a.publicKey, isSigner: false, isWritable: false },
          { pubkey: mint_b.publicKey, isSigner: false, isWritable: false },
          { pubkey: poolAccountA, isSigner: false, isWritable: true },
          { pubkey: poolAccountB, isSigner: false, isWritable: true },
          { pubkey: depositorAccountLp, isSigner: false, isWritable: true },
          { pubkey: depositorAccountA, isSigner: false, isWritable: true },
          { pubkey: depositorAccountB, isSigner: false, isWritable: true },
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
        data: getDepositLiquidityInstructionData(amountA, amountB),
      }),
    );
    tx.recentBlockhash = context.lastBlockhash;
    tx.sign(payer);

    // process the transaction
    await client.processTransaction(tx);
  });

  it('Should withdraw liquidity successfully', async () => {
    const tx = new Transaction();
    tx.add(
      new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: poolPda, isSigner: false, isWritable: true },
          { pubkey: poolAuthorityPda, isSigner: false, isWritable: false },
          { pubkey: mintLiquidityPda, isSigner: false, isWritable: true },
          { pubkey: mint_a.publicKey, isSigner: false, isWritable: false },
          { pubkey: mint_b.publicKey, isSigner: false, isWritable: false },
          { pubkey: poolAccountA, isSigner: false, isWritable: true },
          { pubkey: poolAccountB, isSigner: false, isWritable: true },
          { pubkey: depositorAccountLp, isSigner: false, isWritable: true },
          { pubkey: depositorAccountA, isSigner: false, isWritable: true },
          { pubkey: depositorAccountB, isSigner: false, isWritable: true },
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
        data: getWithdrawLiquidityInstructionData(amountLp),
      }),
    );
    tx.recentBlockhash = context.lastBlockhash;
    tx.sign(payer);

    // process the transaction
    await client.processTransaction(tx);
  });
});
