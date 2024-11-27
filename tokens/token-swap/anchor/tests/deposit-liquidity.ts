import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import { BN } from 'bn.js';
import { expect } from 'chai';
import type { SwapExample } from '../target/types/swap_example';
import { type TestValues, createValues, mintingTokens } from './utils';

describe('Deposit liquidity', () => {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.SwapExample as Program<SwapExample>;

  let values: TestValues;

  beforeEach(async () => {
    values = createValues();

    await program.methods.createAmm(values.id, values.fee).accounts({ amm: values.ammKey, admin: values.admin.publicKey }).rpc();

    await mintingTokens({
      connection,
      creator: values.admin,
      mintAKeypair: values.mintAKeypair,
      mintBKeypair: values.mintBKeypair,
    });

    await program.methods
      .createPool()
      .accounts({
        amm: values.ammKey,
        pool: values.poolKey,
        poolAuthority: values.poolAuthority,
        mintLiquidity: values.mintLiquidity,
        mintA: values.mintAKeypair.publicKey,
        mintB: values.mintBKeypair.publicKey,
        poolAccountA: values.poolAccountA,
        poolAccountB: values.poolAccountB,
      })
      .rpc();
  });

  it('Deposit equal amounts, twice', async () => {
    await program.methods
      .depositLiquidity(values.depositAmountA, values.depositAmountA)
      .accounts({
        pool: values.poolKey,
        poolAuthority: values.poolAuthority,
        depositor: values.admin.publicKey,
        mintLiquidity: values.mintLiquidity,
        mintA: values.mintAKeypair.publicKey,
        mintB: values.mintBKeypair.publicKey,
        poolAccountA: values.poolAccountA,
        poolAccountB: values.poolAccountB,
        depositorAccountLiquidity: values.liquidityAccount,
        depositorAccountA: values.holderAccountA,
        depositorAccountB: values.holderAccountB,
      })
      .signers([values.admin])
      .rpc({ skipPreflight: true });

    const depositTokenAccountLiquidity = await connection.getTokenAccountBalance(values.liquidityAccount);
    expect(depositTokenAccountLiquidity.value.amount).to.equal(values.depositAmountA.sub(values.minimumLiquidity).toString());
    const depositTokenAccountA = await connection.getTokenAccountBalance(values.holderAccountA);
    expect(depositTokenAccountA.value.amount).to.equal(values.defaultHolderAccountSupply.sub(values.depositAmountA).toString());
    const depositTokenAccountB = await connection.getTokenAccountBalance(values.holderAccountB);
    expect(depositTokenAccountB.value.amount).to.equal(values.defaultHolderAccountSupply.sub(values.depositAmountA).toString());

    await program.methods
      .depositLiquidity(values.depositAmountB, values.depositAmountB)
      .accounts({
        pool: values.poolKey,
        poolAuthority: values.poolAuthority,
        depositor: values.admin.publicKey,
        mintLiquidity: values.mintLiquidity,
        mintA: values.mintAKeypair.publicKey,
        mintB: values.mintBKeypair.publicKey,
        poolAccountA: values.poolAccountA,
        poolAccountB: values.poolAccountB,
        depositorAccountLiquidity: values.liquidityAccount,
        depositorAccountA: values.holderAccountA,
        depositorAccountB: values.holderAccountB,
      })
      .signers([values.admin])
      .rpc({ skipPreflight: true });

    const depositTokenAccountLiquidity2 = await connection.getTokenAccountBalance(values.liquidityAccount);
    // No minimumLiquidity subtraction since it's not the first deposit
    expect(depositTokenAccountLiquidity2.value.amount).to.equal(
      new BN(depositTokenAccountLiquidity.value.amount).add(values.depositAmountB).toString(),
    );
    const depositTokenAccountA2 = await connection.getTokenAccountBalance(values.holderAccountA);
    expect(depositTokenAccountA2.value.amount).to.equal(new BN(depositTokenAccountA.value.amount).sub(values.depositAmountB).toString());
    const depositTokenAccountB2 = await connection.getTokenAccountBalance(values.holderAccountB);
    expect(depositTokenAccountB2.value.amount).to.equal(new BN(depositTokenAccountB.value.amount).sub(values.depositAmountB).toString());
  });

  it('Deposit amounts a > b, then a < b', async () => {
    const depositAmountA = new BN(9 * 10 ** 6);
    const depositAmountB = new BN(4 * 10 ** 6);
    await program.methods
      .depositLiquidity(depositAmountA, depositAmountB)
      .accounts({
        pool: values.poolKey,
        poolAuthority: values.poolAuthority,
        depositor: values.admin.publicKey,
        mintLiquidity: values.mintLiquidity,
        mintA: values.mintAKeypair.publicKey,
        mintB: values.mintBKeypair.publicKey,
        poolAccountA: values.poolAccountA,
        poolAccountB: values.poolAccountB,
        depositorAccountLiquidity: values.liquidityAccount,
        depositorAccountA: values.holderAccountA,
        depositorAccountB: values.holderAccountB,
      })
      .signers([values.admin])
      .rpc({ skipPreflight: true });

    const depositTokenAccountLiquidity = await connection.getTokenAccountBalance(values.liquidityAccount);
    expect(depositTokenAccountLiquidity.value.amount).to.equal(new BN(6 * 10 ** 6).sub(values.minimumLiquidity).toString());
    const depositTokenAccountA = await connection.getTokenAccountBalance(values.holderAccountA);
    expect(depositTokenAccountA.value.amount).to.equal(values.defaultHolderAccountSupply.sub(depositAmountA).toString());
    const depositTokenAccountB = await connection.getTokenAccountBalance(values.holderAccountB);
    expect(depositTokenAccountB.value.amount).to.equal(values.defaultHolderAccountSupply.sub(depositAmountB).toString());

    // Expected behavior is that depositAmountA gets increased to
    // (27 * 10 ** 6) * (9/4) = 60.75 * 10 ** 6
    // to maintain the ratio established in the above deposit
    const depositAmountA2 = new BN(18 * 10 ** 6);
    const depositAmountB2 = new BN(27 * 10 ** 6);
    await program.methods
      .depositLiquidity(depositAmountA2, depositAmountB2)
      .accounts({
        pool: values.poolKey,
        poolAuthority: values.poolAuthority,
        depositor: values.admin.publicKey,
        mintLiquidity: values.mintLiquidity,
        mintA: values.mintAKeypair.publicKey,
        mintB: values.mintBKeypair.publicKey,
        poolAccountA: values.poolAccountA,
        poolAccountB: values.poolAccountB,
        depositorAccountLiquidity: values.liquidityAccount,
        depositorAccountA: values.holderAccountA,
        depositorAccountB: values.holderAccountB,
      })
      .signers([values.admin])
      .rpc({ skipPreflight: true });

    const depositTokenAccountLiquidity2 = await connection.getTokenAccountBalance(values.liquidityAccount);
    expect(depositTokenAccountLiquidity2.value.amount).to.equal(
      new BN(40.5 * 10 ** 6).add(new BN(depositTokenAccountLiquidity.value.amount)).toString(),
    );
    const depositTokenAccountA2 = await connection.getTokenAccountBalance(values.holderAccountA);
    expect(depositTokenAccountA2.value.amount).to.equal(new BN(depositTokenAccountA.value.amount).sub(new BN(60.75 * 10 ** 6)).toString());
    const depositTokenAccountB2 = await connection.getTokenAccountBalance(values.holderAccountB);
    expect(depositTokenAccountB2.value.amount).to.equal(new BN(depositTokenAccountB.value.amount).sub(depositAmountB2).toString());
  });

  it('Deposit amounts a < b, then a > b', async () => {
    const depositAmountA = new BN(4 * 10 ** 6);
    const depositAmountB = new BN(9 * 10 ** 6);
    await program.methods
      .depositLiquidity(depositAmountA, depositAmountB)
      .accounts({
        pool: values.poolKey,
        poolAuthority: values.poolAuthority,
        depositor: values.admin.publicKey,
        mintLiquidity: values.mintLiquidity,
        mintA: values.mintAKeypair.publicKey,
        mintB: values.mintBKeypair.publicKey,
        poolAccountA: values.poolAccountA,
        poolAccountB: values.poolAccountB,
        depositorAccountLiquidity: values.liquidityAccount,
        depositorAccountA: values.holderAccountA,
        depositorAccountB: values.holderAccountB,
      })
      .signers([values.admin])
      .rpc({ skipPreflight: true });

    const depositTokenAccountLiquidity = await connection.getTokenAccountBalance(values.liquidityAccount);
    expect(depositTokenAccountLiquidity.value.amount).to.equal(new BN(6 * 10 ** 6).sub(values.minimumLiquidity).toString());
    const depositTokenAccountA = await connection.getTokenAccountBalance(values.holderAccountA);
    expect(depositTokenAccountA.value.amount).to.equal(values.defaultHolderAccountSupply.sub(depositAmountA).toString());
    const depositTokenAccountB = await connection.getTokenAccountBalance(values.holderAccountB);
    expect(depositTokenAccountB.value.amount).to.equal(values.defaultHolderAccountSupply.sub(depositAmountB).toString());

    // Expected behavior is that depositAmountB gets increased to
    // (27 * 10 ** 6) * (9/4) = 60.75 * 10 ** 6
    // to maintain the ratio established in the above deposit
    const depositAmountA2 = new BN(27 * 10 ** 6);
    const depositAmountB2 = new BN(18 * 10 ** 6);
    await program.methods
      .depositLiquidity(depositAmountA2, depositAmountB2)
      .accounts({
        pool: values.poolKey,
        poolAuthority: values.poolAuthority,
        depositor: values.admin.publicKey,
        mintLiquidity: values.mintLiquidity,
        mintA: values.mintAKeypair.publicKey,
        mintB: values.mintBKeypair.publicKey,
        poolAccountA: values.poolAccountA,
        poolAccountB: values.poolAccountB,
        depositorAccountLiquidity: values.liquidityAccount,
        depositorAccountA: values.holderAccountA,
        depositorAccountB: values.holderAccountB,
      })
      .signers([values.admin])
      .rpc({ skipPreflight: true });

    const depositTokenAccountLiquidity2 = await connection.getTokenAccountBalance(values.liquidityAccount);
    expect(depositTokenAccountLiquidity2.value.amount).to.equal(
      new BN(40.5 * 10 ** 6).add(new BN(depositTokenAccountLiquidity.value.amount)).toString(),
    );
    const depositTokenAccountA2 = await connection.getTokenAccountBalance(values.holderAccountA);
    expect(depositTokenAccountA2.value.amount).to.equal(new BN(depositTokenAccountA.value.amount).sub(depositAmountA2).toString());
    const depositTokenAccountB2 = await connection.getTokenAccountBalance(values.holderAccountB);
    expect(depositTokenAccountB2.value.amount).to.equal(new BN(depositTokenAccountB.value.amount).sub(new BN(60.75 * 10 ** 6)).toString());
  });
});
