import type { Program } from "@anchor-lang/core";
import * as anchor from "@anchor-lang/core";
import { expect } from "chai";
import type { SwapExample } from "../target/types/swap_example";
import { createValues, mintingTokens, type TestValues } from "./utils";

describe("Deposit liquidity", () => {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.SwapExample as Program<SwapExample>;

  let values: TestValues;

  beforeEach(async () => {
    values = createValues();

    await program.methods
      .createAmm(values.id, values.fee)
      .accounts({ amm: values.ammKey, admin: values.admin.publicKey })
      .rpc();

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

  it("Deposit liquidity", async () => {
    await program.methods
      .depositLiquidity(values.depositAmountA, values.depositAmountB)
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

    // liquidity = sqrt(amount_a * amount_b) - minimumLiquidity
    const expectedLiquidity =
      Math.floor(Math.sqrt(values.depositAmountA.toNumber() * values.depositAmountB.toNumber())) -
      values.minimumLiquidity.toNumber();

    const depositTokenAccountLiquditiy = await connection.getTokenAccountBalance(values.liquidityAccount);
    expect(depositTokenAccountLiquditiy.value.amount).to.equal(expectedLiquidity.toString());

    const depositTokenAccountA = await connection.getTokenAccountBalance(values.holderAccountA);
    expect(depositTokenAccountA.value.amount).to.equal(values.defaultSupply.sub(values.depositAmountA).toString());

    const depositTokenAccountB = await connection.getTokenAccountBalance(values.holderAccountB);
    // Token B deducted = depositAmountB, not depositAmountA
    expect(depositTokenAccountB.value.amount).to.equal(values.defaultSupply.sub(values.depositAmountB).toString());
  });
});
