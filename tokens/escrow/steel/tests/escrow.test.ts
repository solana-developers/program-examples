import { BN } from "@coral-xyz/anchor";
import { makeKeypairs } from "@solana-developers/helpers";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  MINT_SIZE,
  TOKEN_PROGRAM_ID as TOKEN_PROGRAM,
  createAssociatedTokenAccountIdempotentInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  getMinimumBalanceForRentExemptMint,
} from "@solana/spl-token";
import {
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { BankrunProvider } from "anchor-bankrun";
import { serialize } from "borsh";
import { assert, expect } from "chai";
import { randomBytes } from "node:crypto";
import { before, describe, it } from "node:test";
import { start } from "solana-bankrun";
import * as T from "./types";

const getRandomBigNumber = (size = 8) => {
  return new BN(randomBytes(size));
};

describe("escrow-example", async () => {
  // load program in solana-bankrun
  const PROGRAM_ID = new PublicKey(
    "z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35"
  );
  const context = await start(
    [{ name: "escrow_program", programId: PROGRAM_ID }],
    []
  );
  const provider = new BankrunProvider(context);
  const connection = provider.connection;
  const client = context.banksClient;

  // We're going to reuse these accounts across multiple tests
  const accounts: Record<string, PublicKey> = {
    tokenProgram: TOKEN_PROGRAM,
  };

  const [alice, bob, tokenMintA, tokenMintB] = makeKeypairs(4);

  const tokenAOfferedAmount = new BN(5_000_000);
  const tokenBWantedAmount = new BN(1_000_000);

  before(async () => {
    const [
      aliceTokenAccountA,
      aliceTokenAccountB,
      bobTokenAccountA,
      bobTokenAccountB,
    ] = [alice, bob].flatMap((keypair) =>
      [tokenMintA, tokenMintB].map((mint) =>
        getAssociatedTokenAddressSync(
          mint.publicKey,
          keypair.publicKey,
          false,
          TOKEN_PROGRAM
        )
      )
    );

    // Airdrops to users, and creates two tokens mints 'A' and 'B'"
    const minimumLamports = await getMinimumBalanceForRentExemptMint(
      connection
    );

    const sendSolInstructions: Array<TransactionInstruction> = [alice, bob].map(
      (account) =>
        SystemProgram.transfer({
          fromPubkey: provider.publicKey,
          toPubkey: account.publicKey,
          lamports: 10 * LAMPORTS_PER_SOL,
        })
    );

    const createMintInstructions: Array<TransactionInstruction> = [
      tokenMintA,
      tokenMintB,
    ].map((mint) =>
      SystemProgram.createAccount({
        fromPubkey: provider.publicKey,
        newAccountPubkey: mint.publicKey,
        lamports: minimumLamports,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM,
      })
    );

    // Make tokenA and tokenB mints, mint tokens and create ATAs
    const mintTokensInstructions: Array<TransactionInstruction> = [
      {
        mint: tokenMintA.publicKey,
        authority: alice.publicKey,
        ata: aliceTokenAccountA,
      },
      {
        mint: tokenMintB.publicKey,
        authority: bob.publicKey,
        ata: bobTokenAccountB,
      },
    ].flatMap((mintDetails) => [
      createInitializeMint2Instruction(
        mintDetails.mint,
        6,
        mintDetails.authority,
        null,
        TOKEN_PROGRAM
      ),
      createAssociatedTokenAccountIdempotentInstruction(
        provider.publicKey,
        mintDetails.ata,
        mintDetails.authority,
        mintDetails.mint,
        TOKEN_PROGRAM
      ),
      createMintToInstruction(
        mintDetails.mint,
        mintDetails.ata,
        mintDetails.authority,
        1_000_000_000,
        [],
        TOKEN_PROGRAM
      ),
    ]);

    // Add all these instructions to our transaction
    const tx = new Transaction();
    tx.instructions = [
      ...sendSolInstructions,
      ...createMintInstructions,
      ...mintTokensInstructions,
    ];
    const blockhash = context.lastBlockhash;

    tx.recentBlockhash = blockhash;
    tx.sign(tokenMintA, tokenMintB, alice, bob);
    await provider.sendAndConfirm(tx, [tokenMintA, tokenMintB, alice, bob]);

    // Save the accounts for later use
    accounts.maker = alice.publicKey;
    accounts.taker = bob.publicKey;
    accounts.tokenMintA = tokenMintA.publicKey;
    accounts.makerTokenAccountA = aliceTokenAccountA;
    accounts.takerTokenAccountA = bobTokenAccountA;
    accounts.tokenMintB = tokenMintB.publicKey;
    accounts.makerTokenAccountB = aliceTokenAccountB;
    accounts.takerTokenAccountB = bobTokenAccountB;
  });

  const make = async () => {
    // Pick a random ID for the offer we'll make
    const offerId = getRandomBigNumber();

    // Then determine the account addresses we'll use for the offer and the vault
    const [offer, bump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("offer"),
        accounts.maker.toBuffer(),
        offerId.toArrayLike(Buffer, "le", 8),
      ],
      PROGRAM_ID
    );

    const vault = getAssociatedTokenAddressSync(
      accounts.tokenMintA,
      offer,
      true,
      TOKEN_PROGRAM,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    accounts.offer = offer;
    accounts.vault = vault;

    const data = serialize(
      {
        struct: {
          discriminator: "u8",
          id: "u64",
          token_a_offered_amount: "u64",
          token_b_wanted_amount: "u64",
          bump: "u8",
        },
      },
      {
        discriminator: 0,
        id: offerId,
        token_a_offered_amount: tokenAOfferedAmount,
        token_b_wanted_amount: tokenBWantedAmount,
        bump,
      }
    );

    const ix = new TransactionInstruction({
      data: Buffer.from(data),
      keys: [
        { pubkey: alice.publicKey, isSigner: true, isWritable: true },
        { pubkey: accounts.tokenMintA, isSigner: false, isWritable: true },
        { pubkey: accounts.tokenMintB, isSigner: false, isWritable: true },
        {
          pubkey: accounts.makerTokenAccountA,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: accounts.offer,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: accounts.vault,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: TOKEN_PROGRAM,
          isSigner: false,
          isWritable: false,
        },
        {
          pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
    });

    const blockhash = context.lastBlockhash;
    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix);
    tx.sign(alice);
    await client.processTransaction(tx);

    // Check our vault contains the tokens offered
    const vaultBalanceResponse = await connection.getAccountInfo(vault);
    const datas = T.decodeAccount(vaultBalanceResponse.data);
    const vaultBalance = new BN(datas.amount.toString());
    assert(vaultBalance.eq(tokenAOfferedAmount));

    // Check our Offer account contains the correct data
    const offerAccount = await connection.getAccountInfo(offer);
    const offerData = T.decodeOffer(offerAccount.data);

    assert(offerData.maker.equals(alice.publicKey));
    assert(offerData.tokenMintA.equals(accounts.tokenMintA));
    assert(offerData.tokenMintB.equals(accounts.tokenMintB));
    const tokenBWanted = new BN(offerData.tokenBWantedAmount.toString());
    assert(tokenBWanted.eq(tokenBWantedAmount));
  };

  const take = async () => {
    const ix = new TransactionInstruction({
      data: Buffer.alloc(1, 1),
      keys: [
        { pubkey: bob.publicKey, isSigner: true, isWritable: true },
        { pubkey: alice.publicKey, isSigner: false, isWritable: !true },
        { pubkey: accounts.tokenMintA, isSigner: false, isWritable: true },
        { pubkey: accounts.tokenMintB, isSigner: false, isWritable: true },
        {
          pubkey: accounts.takerTokenAccountA,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: accounts.takerTokenAccountB,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: accounts.makerTokenAccountB,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: accounts.offer,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: accounts.vault,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: TOKEN_PROGRAM,
          isSigner: false,
          isWritable: false,
        },
        {
          pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
    });

    const blockhash = context.lastBlockhash;
    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix);
    tx.sign(bob);
    await client.processTransaction(tx);

    /**Check alice's account for the tokens wanted*/
    const AliceAccountData = T.decodeAccount(
      (await connection.getAccountInfo(accounts.makerTokenAccountB)).data
    );
    const aliceBalance = new BN(AliceAccountData.amount.toString());
    assert(aliceBalance.eq(tokenBWantedAmount));

    /**Check bobs's account for the tokens offered*/
    const bobsData = T.decodeAccount(
      (await connection.getAccountInfo(accounts.takerTokenAccountA)).data
    );
    const bobsBalance = new BN(bobsData.amount.toString());
    assert.strictEqual(bobsBalance.toString(), tokenAOfferedAmount.toString());
  };

  it("Puts the tokens Alice offers into the vault when Alice makes an offer", async () => {
    await make();
  });

  it("Puts the tokens from the vault into Bob's account, and gives Alice Bob's tokens, when Bob takes an offer", async () => {
    await take();
    /**Check that the Vault account does not exist */

    try {
      await connection.getAccountInfo(accounts.vault);
    } catch (error: any) {
      expect(error.toString()).to.include(
        `Could not find ${accounts.vault.toString()}`
      );
    }
  });

  it("Refunds the token amount from the vault if the maker chooses to", async () => {
    await make();
    const ix = new TransactionInstruction({
      data: Buffer.alloc(1, 2),
      keys: [
        { pubkey: alice.publicKey, isSigner: true, isWritable: true },
        { pubkey: accounts.tokenMintA, isSigner: false, isWritable: true },
        {
          pubkey: accounts.makerTokenAccountA,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: accounts.offer,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: accounts.vault,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: TOKEN_PROGRAM,
          isSigner: false,
          isWritable: false,
        },
        {
          pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId: PROGRAM_ID,
    });

    const blockhash = context.lastBlockhash;
    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix);
    tx.sign(alice);
    await client.processTransaction(tx);

    /**Check alice's account for the tokens wanted*/
    const AliceAccountData = T.decodeAccount(
      (await connection.getAccountInfo(accounts.makerTokenAccountA)).data
    );
    const aliceBalance = new BN(AliceAccountData.amount.toString());
    expect(aliceBalance.toString()).to.equal(
      new BN(1_000_000_000).sub(tokenAOfferedAmount).toString()
    );

    /**Check that the Vault account does not exist */
    try {
      await connection.getAccountInfo(accounts.vault);
    } catch (error: any) {
      expect(error.toString()).to.include(
        `Could not find ${accounts.vault.toString()}`
      );
    }
  });

  it("should fail when Bob tries to withdraw Alices funds without depositing", async () => {
    await make();
    const ix = new TransactionInstruction({
      data: Buffer.alloc(1, 2),
      keys: [
        { pubkey: bob.publicKey, isSigner: true, isWritable: true },
        { pubkey: accounts.tokenMintA, isSigner: false, isWritable: true },
        {
          pubkey: accounts.takerTokenAccountA,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: accounts.offer,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: accounts.vault,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: TOKEN_PROGRAM,
          isSigner: false,
          isWritable: false,
        },
        {
          pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId: PROGRAM_ID,
    });

    const blockhash = context.lastBlockhash;
    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix);
    tx.sign(bob);

    try {
      await client.processTransaction(tx);
    } catch (error) {
      expect(error.message).to.include("Error");
    }
  });
});
