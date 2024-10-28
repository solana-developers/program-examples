import { BankrunProvider } from "anchor-bankrun";
import { expect } from "chai";
import { beforeEach, describe, it } from "node:test";
import { AccountInfoBytes, start } from "solana-bankrun";
import {
  createAmmTransactionInstruction,
  createPoolInstruction,
} from "./transactions";
import { Pool, PoolLayout } from "./types";
import {
  createValues,
  expectRevert,
  mintingTokens,
  PROGRAM_ID,
  TestValues,
} from "./utils";

describe("Testing Creaion of Pool", async () => {
  const context = await start(
    [{ name: "token_swap_program", programId: PROGRAM_ID }],
    []
  );
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
  });

  it("creates a pool successfully", async () => {
    let tx = createAmmTransactionInstruction(values, payer, context);
    await client.processTransaction(tx);
    tx = createPoolInstruction(values, payer, context);
    await client.processTransaction(tx);

    let poolAccount: Pool | AccountInfoBytes = await client.getAccount(
      values.poolKey
    );
    poolAccount = PoolLayout.decode(poolAccount.data);

    expect(poolAccount.amm.toString()).to.equal(values.ammKey.toString());
    expect(poolAccount.mintA.toString()).to.equal(
      values.mintAKeypair.publicKey.toString()
    );
    expect(poolAccount.mintB.toString()).to.equal(
      values.mintBKeypair.publicKey.toString()
    );
  });

  it("fails to create pool with invalid mints", async () => {
    let tx = createAmmTransactionInstruction(values, payer, context);
    await client.processTransaction(tx);
    tx = createPoolInstruction(values, payer, context, true);
    let reverted = await expectRevert(client.processTransaction(tx));
    expect(reverted, "This transaction should fail but it passed").to.be.true;
  });
});
