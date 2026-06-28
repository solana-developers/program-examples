import { AccountLayout, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import BN from "bn.js";
import * as borsh from "borsh";
import { assert } from "chai";
import { start } from "solana-bankrun";
import { type ContributorRaw, ContributorSchema, type FundraiserRaw, FundraiserSchema } from "./account";
import { buildCheckContributions, buildContribute, buildInitialize, buildRefund } from "./instruction";
import { createValues, DECIMALS, expectRevert, mintingTokens } from "./utils";

describe("Fundraiser (Pinocchio)", async () => {
  const values = createValues();

  const context = await start([{ name: "fundraiser_pinocchio_program", programId: values.programId }], []);
  const client = context.banksClient;
  const payer = context.payer;

  // The fee payer doubles as the contributor.
  const contributor = payer;
  const contributorAta = getAssociatedTokenAddressSync(values.mintKeypair.publicKey, contributor.publicKey, true);
  const [contributorAccount, contributorBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("contributor"), values.fundraiser.toBuffer(), contributor.publicKey.toBuffer()],
    values.programId,
  );

  const oneToken = new BN(10 ** DECIMALS);

  // Give the maker some SOL to fund the fundraiser and vault accounts, then mint
  // 100 tokens to the contributor.
  {
    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(
      SystemProgram.transfer({
        fromPubkey: payer.publicKey,
        toPubkey: values.maker.publicKey,
        lamports: LAMPORTS_PER_SOL,
      }),
    ).sign(payer);
    await client.processTransaction(tx);
  }
  await mintingTokens({ context, holder: contributor, mintKeypair: values.mintKeypair });

  it("Initializes a fundraiser", async () => {
    const ix = buildInitialize({
      amount: values.amountToRaise,
      duration: values.duration,
      bump: values.fundraiserBump,
      maker: values.maker.publicKey,
      mint: values.mintKeypair.publicKey,
      fundraiser: values.fundraiser,
      vault: values.vault,
      programId: values.programId,
    });

    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix).sign(payer, values.maker);
    await client.processTransaction(tx);

    const fundraiserInfo = await client.getAccount(values.fundraiser);
    if (fundraiserInfo === null) throw new Error("Fundraiser account not found");
    const fundraiser = borsh.deserialize(FundraiserSchema, Buffer.from(fundraiserInfo.data)) as FundraiserRaw;

    assert(
      new PublicKey(fundraiser.maker).toBase58() === values.maker.publicKey.toBase58(),
      "maker key does not match",
    );
    assert(
      new PublicKey(fundraiser.mint_to_raise).toBase58() === values.mintKeypair.publicKey.toBase58(),
      "wrong mint",
    );
    assert(fundraiser.amount_to_raise.toString() === values.amountToRaise.toString(), "wrong target amount");
    assert(fundraiser.current_amount.toString() === "0", "current amount should start at 0");
    assert(fundraiser.duration === values.duration, "wrong duration");
    assert(fundraiser.bump === values.fundraiserBump, "wrong bump");
    assert(new PublicKey(fundraiser.vault).toBase58() === values.vault.toBase58(), "wrong vault recorded");

    const vaultInfo = await client.getAccount(values.vault);
    if (vaultInfo === null) throw new Error("Vault account not found");
    const vault = AccountLayout.decode(vaultInfo.data);
    assert(vault.amount.toString() === "0", "vault should start empty");
  });

  it("Accepts a contribution", async () => {
    const ix = buildContribute({
      amount: oneToken,
      bump: contributorBump,
      contributor: contributor.publicKey,
      mint: values.mintKeypair.publicKey,
      fundraiser: values.fundraiser,
      contributorAccount,
      contributorAta,
      vault: values.vault,
      programId: values.programId,
    });

    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix).sign(payer);
    await client.processTransaction(tx);

    const vaultInfo = await client.getAccount(values.vault);
    if (vaultInfo === null) throw new Error("Vault account not found");
    assert(AccountLayout.decode(vaultInfo.data).amount.toString() === oneToken.toString(), "wrong vault amount");

    const contributorInfo = await client.getAccount(contributorAccount);
    if (contributorInfo === null) throw new Error("Contributor account not found");
    const contributorState = borsh.deserialize(ContributorSchema, Buffer.from(contributorInfo.data)) as ContributorRaw;
    assert(contributorState.amount.toString() === oneToken.toString(), "wrong contributor amount");
    assert(contributorState.bump === contributorBump, "wrong contributor bump");

    const fundraiserInfo = await client.getAccount(values.fundraiser);
    if (fundraiserInfo === null) throw new Error("Fundraiser account not found");
    const fundraiser = borsh.deserialize(FundraiserSchema, Buffer.from(fundraiserInfo.data)) as FundraiserRaw;
    assert(fundraiser.current_amount.toString() === oneToken.toString(), "wrong current amount");
  });

  it("Accepts a second contribution", async () => {
    const ix = buildContribute({
      amount: oneToken,
      bump: contributorBump,
      contributor: contributor.publicKey,
      mint: values.mintKeypair.publicKey,
      fundraiser: values.fundraiser,
      contributorAccount,
      contributorAta,
      vault: values.vault,
      programId: values.programId,
    });

    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix).sign(payer);
    await client.processTransaction(tx);

    const vaultInfo = await client.getAccount(values.vault);
    if (vaultInfo === null) throw new Error("Vault account not found");
    const expected = oneToken.muln(2).toString();
    assert(AccountLayout.decode(vaultInfo.data).amount.toString() === expected, "wrong vault amount");

    const contributorInfo = await client.getAccount(contributorAccount);
    if (contributorInfo === null) throw new Error("Contributor account not found");
    const contributorState = borsh.deserialize(ContributorSchema, Buffer.from(contributorInfo.data)) as ContributorRaw;
    assert(contributorState.amount.toString() === expected, "wrong contributor amount");
  });

  it("Rejects a contribution above the per-contributor cap", async () => {
    // The cap is 10% of the 30-token target = 3 tokens. Two tokens are already
    // contributed, so a further two tokens (total 4) must be rejected.
    const ix = buildContribute({
      amount: oneToken.muln(2),
      bump: contributorBump,
      contributor: contributor.publicKey,
      mint: values.mintKeypair.publicKey,
      fundraiser: values.fundraiser,
      contributorAccount,
      contributorAta,
      vault: values.vault,
      programId: values.programId,
    });

    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix).sign(payer);
    await expectRevert(client.processTransaction(tx));
  });

  it("Rejects checking contributions before the target is met", async () => {
    const ix = buildCheckContributions({
      maker: values.maker.publicKey,
      mint: values.mintKeypair.publicKey,
      fundraiser: values.fundraiser,
      vault: values.vault,
      makerAta: values.makerAta,
      programId: values.programId,
    });

    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix).sign(payer, values.maker);
    await expectRevert(client.processTransaction(tx));
  });

  it("Refunds the contributor", async () => {
    const beforeInfo = await client.getAccount(contributorAta);
    if (beforeInfo === null) throw new Error("Contributor token account not found");
    const before = AccountLayout.decode(beforeInfo.data).amount;

    const ix = buildRefund({
      contributor: contributor.publicKey,
      maker: values.maker.publicKey,
      mint: values.mintKeypair.publicKey,
      fundraiser: values.fundraiser,
      contributorAccount,
      contributorAta,
      vault: values.vault,
      programId: values.programId,
    });

    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix).sign(payer);
    await client.processTransaction(tx);

    // The contributor account is closed.
    const contributorInfo = await client.getAccount(contributorAccount);
    assert(contributorInfo === null, "contributor account not closed");

    // The vault is drained back to the contributor.
    const vaultInfo = await client.getAccount(values.vault);
    if (vaultInfo === null) throw new Error("Vault account not found");
    assert(AccountLayout.decode(vaultInfo.data).amount.toString() === "0", "vault should be empty after refund");

    const afterInfo = await client.getAccount(contributorAta);
    if (afterInfo === null) throw new Error("Contributor token account not found");
    const after = AccountLayout.decode(afterInfo.data).amount;
    assert((after - before).toString() === oneToken.muln(2).toString(), "contributor not fully refunded");

    // The fundraiser's running total is back to zero.
    const fundraiserInfo = await client.getAccount(values.fundraiser);
    if (fundraiserInfo === null) throw new Error("Fundraiser account not found");
    const fundraiser = borsh.deserialize(FundraiserSchema, Buffer.from(fundraiserInfo.data)) as FundraiserRaw;
    assert(fundraiser.current_amount.toString() === "0", "current amount should be zero after refund");
  });
});
