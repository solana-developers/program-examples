import { PublicKey } from "@solana/web3.js";
import { BankrunProvider } from "anchor-bankrun";
import { assert, expect } from "chai";
import { beforeEach, describe, it } from "node:test";
import { AccountInfoBytes, start } from "solana-bankrun";
import { createAmmTransactionInstruction } from "./transactions";
import { Amm, AmmLayout } from "./types";
import { createValues, expectRevert, PROGRAM_ID, TestValues } from "./utils";

describe("Testing Creation of amm", async () => {
  const context = await start(
    [{ name: "token_swap_program", programId: PROGRAM_ID }],
    []
  );
  const provider = new BankrunProvider(context);
  const connection = provider.connection;
  const client = context.banksClient;
  const payer = context.payer;
  let values: TestValues;

  beforeEach(() => {
    values = createValues();
  });

  it("creates an amm successfully", async () => {
    const tx = createAmmTransactionInstruction(values, payer, context);
    await client.processTransaction(tx);

    let ammAccount: Amm | AccountInfoBytes = await client.getAccount(
      values.ammKey
    );
    ammAccount = AmmLayout.decode(ammAccount.data);

    expect(ammAccount.id.toString()).to.equal(values.id.toString());
    expect(ammAccount.admin.toString()).to.equal(
      values.admin.publicKey.toString()
    );
    expect(ammAccount.fee.toString()).to.equal(values.fee.toString());
  });

  it("failed to create an amm due to fee too high", async () => {
    const tx = createAmmTransactionInstruction(values, payer, context, true);
    let reverted = await expectRevert(client.processTransaction(tx));
    expect(reverted, "This transaction should fail but it passed").to.be.true;
  });
});
