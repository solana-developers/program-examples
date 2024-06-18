import { randomBytes } from "node:crypto";
import * as anchor from "@coral-xyz/anchor";
import { BN, Wallet, type Program } from "@coral-xyz/anchor";
import {
  TOKEN_2022_PROGRAM_ID,
  type TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { LAMPORTS_PER_SOL, PublicKey, Keypair } from "@solana/web3.js";
import { assert } from "chai";
import type { Escrow } from "../target/types/escrow";

import {
  confirmTransaction,
  createAccountsMintsAndTokenAccounts,
} from "@solana-developers/helpers";

const TOKEN_PROGRAM: typeof TOKEN_2022_PROGRAM_ID | typeof TOKEN_PROGRAM_ID =
  TOKEN_2022_PROGRAM_ID;

const getRandomBigNumber = (size = 8) => {
  return new BN(randomBytes(size));
};

describe("escrow", async () => {
  // Use the cluster and the keypair from Anchor.toml
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const connection = provider.connection;

  const program = anchor.workspace.Escrow as Program<Escrow>;

  // We're going to reuse these accounts across multiple tests
  const accounts: Record<string, PublicKey> = {
    tokenProgram: TOKEN_PROGRAM,
  };

  let alice: Keypair;
  let bob: Keypair;
  let tokenMintA: Keypair;
  let tokenMintB: Keypair;

  before(
    "Creates Alice and Bob accounts, 2 token mints, and associated token accounts for both tokens for both users",
    async () => {
      const connection = provider.connection;

      const providerUser = (provider.wallet as Wallet).payer;

      const SOL_BALANCE = 10 * LAMPORTS_PER_SOL;

      const accountsMintsAndTokenAccounts =
        await createAccountsMintsAndTokenAccounts(
          [
            [1_000_000_000, 0], // Alice has 1_000_000_000 of token A and 0 of token B
            [0, 1_000_000_000], // Bob has 0 of token A and 1_000_000_000 of token B
          ],
          SOL_BALANCE,
          connection,
          providerUser
        );

      const tokenAccounts = accountsMintsAndTokenAccounts.tokenAccounts;

      [alice, bob] = accountsMintsAndTokenAccounts.users;
      [tokenMintA, tokenMintB] = accountsMintsAndTokenAccounts.mints;

      // Save the accounts for later use
      accounts.maker = alice.publicKey;
      accounts.taker = bob.publicKey;
      accounts.tokenMintA = tokenMintA.publicKey;
      accounts.makerTokenAccountA = tokenAccounts[0][0];
      accounts.takerTokenAccountA = tokenAccounts[1][0];
      accounts.tokenMintB = tokenMintB.publicKey;
      accounts.makerTokenAccountB = tokenAccounts[0][1];
      accounts.takerTokenAccountB = tokenAccounts[1][1];
    }
  );

  const tokenAOfferedAmount = new BN(1_000_000);
  const tokenBWantedAmount = new BN(1_000_000);

  // We'll call this function from multiple tests, so let's seperate it out
  const make = async () => {
    // Pick a random ID for the offer we'll make
    const offerId = getRandomBigNumber();

    // Then determine the account addresses we'll use for the offer and the vault
    const offer = PublicKey.findProgramAddressSync(
      [
        Buffer.from("offer"),
        accounts.maker.toBuffer(),
        offerId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];

    const vault = getAssociatedTokenAddressSync(
      accounts.tokenMintA,
      offer,
      true,
      TOKEN_PROGRAM
    );

    accounts.offer = offer;
    accounts.vault = vault;

    const transactionSignature = await program.methods
      .makeOffer(offerId, tokenAOfferedAmount, tokenBWantedAmount)
      .accounts({ ...accounts })
      .signers([alice])
      .rpc();

    await confirmTransaction(connection, transactionSignature);

    // Check our vault contains the tokens offered
    const vaultBalanceResponse = await connection.getTokenAccountBalance(vault);
    const vaultBalance = new BN(vaultBalanceResponse.value.amount);
    assert(vaultBalance.eq(tokenAOfferedAmount));

    // Check our Offer account contains the correct data
    const offerAccount = await program.account.offer.fetch(offer);

    assert(offerAccount.maker.equals(alice.publicKey));
    assert(offerAccount.tokenMintA.equals(accounts.tokenMintA));
    assert(offerAccount.tokenMintB.equals(accounts.tokenMintB));
    assert(offerAccount.tokenBWantedAmount.eq(tokenBWantedAmount));
  };

  // We'll call this function from multiple tests, so let's seperate it out
  const take = async () => {
    const transactionSignature = await program.methods
      .takeOffer()
      .accounts({ ...accounts })
      .signers([bob])
      .rpc();

    await confirmTransaction(connection, transactionSignature);

    // Check the offered tokens are now in Bob's account
    // (note: there is no before balance as Bob didn't have any offered tokens before the transaction)
    const bobTokenAccountBalanceAfterResponse =
      await connection.getTokenAccountBalance(accounts.takerTokenAccountA);
    const bobTokenAccountBalanceAfter = new BN(
      bobTokenAccountBalanceAfterResponse.value.amount
    );
    assert(bobTokenAccountBalanceAfter.eq(tokenAOfferedAmount));

    // Check the wanted tokens are now in Alice's account
    // (note: there is no before balance as Alice didn't have any wanted tokens before the transaction)
    const aliceTokenAccountBalanceAfterResponse =
      await connection.getTokenAccountBalance(accounts.makerTokenAccountB);
    const aliceTokenAccountBalanceAfter = new BN(
      aliceTokenAccountBalanceAfterResponse.value.amount
    );
    assert(aliceTokenAccountBalanceAfter.eq(tokenBWantedAmount));
  };

  it("Puts the tokens Alice offers into the vault when Alice makes an offer", async () => {
    await make();
  });

  it("Puts the tokens from the vault into Bob's account, and gives Alice Bob's tokens, when Bob takes an offer", async () => {
    await take();
  });
});
