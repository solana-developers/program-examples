import {
  AccountLayout,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountIdempotentInstruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, Transaction } from "@solana/web3.js";
import BN from "bn.js";
import * as borsh from "borsh";
import { assert } from "chai";
import { start } from "solana-bankrun";
import { type ContributorRaw, ContributorSchema, type FundraiserRaw, FundraiserSchema } from "./account";
import { buildCheckContributions, buildContribute, buildInitialize, buildRefund } from "./instruction";
import { expectRevert, fundAccount, mintingTokens } from "./utils";

describe("Token Fundraiser (Pinocchio)", async () => {
  const programId = PublicKey.unique();
  const context = await start([{ name: "token_fundraiser_pinocchio_program", programId }], []);
  const client = context.banksClient;
  const payer = context.payer;
  // The bankrun payer plays the role of the contributor.
  const contributor = payer;

  const maker = Keypair.generate();
  const mintKeypair = Keypair.generate();

  const [fundraiser, fundraiserBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("fundraiser"), maker.publicKey.toBuffer()],
    programId,
  );
  const [contributorAccount, contributorBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("contributor"), fundraiser.toBuffer(), contributor.publicKey.toBuffer()],
    programId,
  );

  const vault = getAssociatedTokenAddressSync(mintKeypair.publicKey, fundraiser, true);
  const contributorAta = getAssociatedTokenAddressSync(mintKeypair.publicKey, contributor.publicKey);
  const makerAta = getAssociatedTokenAddressSync(mintKeypair.publicKey, maker.publicKey);

  const decimals = 6;
  const amountToRaise = new BN(30_000_000); // 30 tokens
  const duration = 5; // an open, 5-day campaign

  // Fund the maker so it can pay for the fundraiser + vault, and mint tokens to
  // the contributor.
  await fundAccount(context, maker.publicKey, 5 * LAMPORTS_PER_SOL);
  await mintingTokens({ context, holder: contributor, mintKeypair, mintedAmount: 10, decimals });

  async function sendInstruction(ix, signers: Keypair[]) {
    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix).sign(...signers);
    await client.processTransaction(tx);
  }

  async function readTokenAmount(account: PublicKey): Promise<bigint> {
    const info = await client.getAccount(account);
    if (info === null) throw new Error("Token account not found");
    return AccountLayout.decode(info.data).amount;
  }

  it("Initializes a fundraiser", async () => {
    const ix = buildInitialize({
      amount: amountToRaise,
      duration,
      bump: fundraiserBump,
      maker: maker.publicKey,
      mint: mintKeypair.publicKey,
      fundraiser,
      vault,
      programId,
    });

    await sendInstruction(ix, [payer, maker]);

    const info = await client.getAccount(fundraiser);
    if (info === null) throw new Error("Fundraiser account not found");
    const state = borsh.deserialize(FundraiserSchema, Buffer.from(info.data)) as FundraiserRaw;

    assert.equal(new PublicKey(state.maker).toBase58(), maker.publicKey.toBase58(), "wrong maker");
    assert.equal(
      new PublicKey(state.mint_to_raise).toBase58(),
      mintKeypair.publicKey.toBase58(),
      "wrong mint",
    );
    assert.equal(state.amount_to_raise.toString(), amountToRaise.toString(), "wrong target");
    assert.equal(state.current_amount.toString(), "0", "current amount should start at zero");
    assert.equal(state.duration, duration, "wrong duration");
    assert.equal(state.bump, fundraiserBump, "wrong bump");

    // The vault exists and starts empty.
    assert.equal((await readTokenAmount(vault)).toString(), "0", "vault should start empty");
  });

  it("Accepts contributions while the campaign is open", async () => {
    for (let i = 0; i < 2; i++) {
      const ix = buildContribute({
        amount: new BN(1_000_000),
        contributor_bump: contributorBump,
        contributor: contributor.publicKey,
        mint: mintKeypair.publicKey,
        fundraiser,
        contributorAccount,
        contributorAta,
        vault,
        programId,
      });
      await sendInstruction(ix, [payer]);
    }

    assert.equal((await readTokenAmount(vault)).toString(), "2000000", "vault should hold both contributions");

    const info = await client.getAccount(contributorAccount);
    if (info === null) throw new Error("Contributor account not found");
    const state = borsh.deserialize(ContributorSchema, Buffer.from(info.data)) as ContributorRaw;
    assert.equal(state.amount.toString(), "2000000", "contributor total should be tracked");

    const fundraiserInfo = await client.getAccount(fundraiser);
    if (fundraiserInfo === null) throw new Error("Fundraiser account not found");
    const fundraiserState = borsh.deserialize(FundraiserSchema, Buffer.from(fundraiserInfo.data)) as FundraiserRaw;
    assert.equal(fundraiserState.current_amount.toString(), "2000000", "fundraiser total should be tracked");
  });

  it("Rejects a contribution whose vault is not owned by the fundraiser", async () => {
    // Passing the contributor's own token account as the vault must be rejected;
    // otherwise the contributor could keep their tokens while still inflating the
    // recorded total and later drain the real vault via a refund.
    const ix = buildContribute({
      amount: new BN(1_000_000),
      contributor_bump: contributorBump,
      contributor: contributor.publicKey,
      mint: mintKeypair.publicKey,
      fundraiser,
      contributorAccount,
      contributorAta,
      vault: contributorAta, // not the fundraiser's vault
      programId,
    });

    await expectRevert(sendInstruction(ix, [payer]));
  });

  it("Rejects a contribution above the per-contributor maximum", async () => {
    // The contributor is already at 2_000_000; the cap is 10% of 30_000_000 =
    // 3_000_000, so another 2_000_000 must be rejected.
    const ix = buildContribute({
      amount: new BN(2_000_000),
      contributor_bump: contributorBump,
      contributor: contributor.publicKey,
      mint: mintKeypair.publicKey,
      fundraiser,
      contributorAccount,
      contributorAta,
      vault,
      programId,
    });

    await expectRevert(sendInstruction(ix, [payer]));
  });

  it("Rejects settling before the target is met", async () => {
    const ix = buildCheckContributions({
      maker: maker.publicKey,
      mint: mintKeypair.publicKey,
      fundraiser,
      vault,
      makerAta,
      programId,
    });

    await expectRevert(sendInstruction(ix, [payer, maker]));
  });

  it("Rejects a refund while the campaign is still open", async () => {
    // The 5-day campaign has just started, so a refund must be rejected.
    const ix = buildRefund({
      contributor_bump: contributorBump,
      contributor: contributor.publicKey,
      maker: maker.publicKey,
      mint: mintKeypair.publicKey,
      fundraiser,
      contributorAccount,
      contributorAta,
      vault,
      programId,
    });

    await expectRevert(sendInstruction(ix, [payer]));
  });

  it("Rejects a contribution after the campaign has ended", async () => {
    // A second fundraiser created with duration = 0 is already over the moment
    // it is initialized, so any contribution must be rejected.
    const maker2 = Keypair.generate();
    await fundAccount(context, maker2.publicKey, 5 * LAMPORTS_PER_SOL);

    const [endedFundraiser, endedBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("fundraiser"), maker2.publicKey.toBuffer()],
      programId,
    );
    const endedVault = getAssociatedTokenAddressSync(mintKeypair.publicKey, endedFundraiser, true);
    const [endedContributor, endedContributorBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("contributor"), endedFundraiser.toBuffer(), contributor.publicKey.toBuffer()],
      programId,
    );

    await sendInstruction(
      buildInitialize({
        amount: amountToRaise,
        duration: 0,
        bump: endedBump,
        maker: maker2.publicKey,
        mint: mintKeypair.publicKey,
        fundraiser: endedFundraiser,
        vault: endedVault,
        programId,
      }),
      [payer, maker2],
    );

    const ix = buildContribute({
      amount: new BN(1_000_000),
      contributor_bump: endedContributorBump,
      contributor: contributor.publicKey,
      mint: mintKeypair.publicKey,
      fundraiser: endedFundraiser,
      contributorAccount: endedContributor,
      contributorAta,
      vault: endedVault,
      programId,
    });

    await expectRevert(sendInstruction(ix, [payer]));
  });

  it("Settles a fully-funded campaign and reclaims rent", async () => {
    // A fresh campaign funded to its target by 10 contributors (each capped at
    // 10% of the target). Settling transfers everything to the maker and closes
    // both the vault and the fundraiser account.
    const settleMaker = Keypair.generate();
    await fundAccount(context, settleMaker.publicKey, 5 * LAMPORTS_PER_SOL);

    const [settleFundraiser, settleBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("fundraiser"), settleMaker.publicKey.toBuffer()],
      programId,
    );
    const settleVault = getAssociatedTokenAddressSync(mintKeypair.publicKey, settleFundraiser, true);
    const settleMakerAta = getAssociatedTokenAddressSync(mintKeypair.publicKey, settleMaker.publicKey);

    await sendInstruction(
      buildInitialize({
        amount: amountToRaise, // 30_000_000
        duration: 5,
        bump: settleBump,
        maker: settleMaker.publicKey,
        mint: mintKeypair.publicKey,
        fundraiser: settleFundraiser,
        vault: settleVault,
        programId,
      }),
      [payer, settleMaker],
    );

    // 10 contributors x 3_000_000 (the per-contributor cap) == the 30_000_000 target.
    for (let i = 0; i < 10; i++) {
      const c = Keypair.generate();
      await fundAccount(context, c.publicKey, 1 * LAMPORTS_PER_SOL);
      const cAta = getAssociatedTokenAddressSync(mintKeypair.publicKey, c.publicKey);

      const fundTx = new Transaction();
      fundTx.recentBlockhash = context.lastBlockhash;
      fundTx
        .add(
          createAssociatedTokenAccountIdempotentInstruction(
            payer.publicKey,
            cAta,
            c.publicKey,
            mintKeypair.publicKey,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID,
          ),
          createMintToInstruction(mintKeypair.publicKey, cAta, payer.publicKey, 3_000_000, [], TOKEN_PROGRAM_ID),
        )
        .sign(payer);
      await client.processTransaction(fundTx);

      const [cAccount, cBump] = PublicKey.findProgramAddressSync(
        [Buffer.from("contributor"), settleFundraiser.toBuffer(), c.publicKey.toBuffer()],
        programId,
      );
      await sendInstruction(
        buildContribute({
          amount: new BN(3_000_000),
          contributor_bump: cBump,
          contributor: c.publicKey,
          mint: mintKeypair.publicKey,
          fundraiser: settleFundraiser,
          contributorAccount: cAccount,
          contributorAta: cAta,
          vault: settleVault,
          programId,
        }),
        [payer, c],
      );
    }

    assert.equal((await readTokenAmount(settleVault)).toString(), "30000000", "vault should hold the full target");

    await sendInstruction(
      buildCheckContributions({
        maker: settleMaker.publicKey,
        mint: mintKeypair.publicKey,
        fundraiser: settleFundraiser,
        vault: settleVault,
        makerAta: settleMakerAta,
        programId,
      }),
      [payer, settleMaker],
    );

    // The maker received the full raised amount, and the vault + fundraiser are closed.
    assert.equal((await readTokenAmount(settleMakerAta)).toString(), "30000000", "maker should receive the raised funds");
    assert.equal(await client.getAccount(settleVault), null, "vault should be closed after settlement");
    assert.equal(await client.getAccount(settleFundraiser), null, "fundraiser should be closed after settlement");
  });
});
