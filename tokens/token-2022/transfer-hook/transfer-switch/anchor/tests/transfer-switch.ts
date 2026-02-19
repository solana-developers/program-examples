import { describe, it } from "node:test";
import * as anchor from "@coral-xyz/anchor";
import {
  AccountLayout,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  createInitializeMintInstruction,
  createInitializeTransferHookInstruction,
  createMintToInstruction,
  createTransferCheckedWithTransferHookInstruction,
  ExtensionType,
  getAssociatedTokenAddressSync,
  getMintLen,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,  LAMPORTS_PER_SOL} from "@solana/web3.js";
import { LiteSVMProvider } from 'anchor-litesvm';
import { assert } from "chai";
import { LiteSVM } from 'litesvm';
import type { TransferSwitch } from "../target/types/transfer_switch";

import IDL from "../target/idl/transfer_switch.json";
const PROGRAM_ID = new PublicKey(IDL.address);

const expectRevert = async (promise: Promise<any>) => {
  try {
    await promise;
    throw new Error("Expected a revert");
  } catch {
    return;
  }
};

describe("Transfer switch", async () => {
  const svm = new LiteSVM();
  svm.addProgramFromFile(PROGRAM_ID, 'target/deploy/transfer_switch.so');
  const payer = Keypair.generate();
  svm.airdrop(payer.publicKey, BigInt(100 * LAMPORTS_PER_SOL));
  const provider = new LiteSVMProvider(svm, new anchor.Wallet(payer));

  const _wallet = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<TransferSwitch>(IDL, provider);
  const connection = provider.connection;

  // payer is already defined above as the Keypair used in the wallet

  // Generate keypair to use as address for the transfer-hook enabled mint
  const mint = Keypair.generate();
  const decimals = 9;

  function newUser(): [Keypair, PublicKey, TransactionInstruction] {
    const user = Keypair.generate();
    const userTokenAccount = getAssociatedTokenAddressSync(
      mint.publicKey,
      user.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID,
    );
    const createUserTokenAccountIx = createAssociatedTokenAccountInstruction(
      payer.publicKey,
      userTokenAccount,
      user.publicKey,
      mint.publicKey,
      TOKEN_2022_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID,
    );

    return [user, userTokenAccount, createUserTokenAccountIx];
  }

  // admin config address
  const adminConfigAddress = PublicKey.findProgramAddressSync(
    [Buffer.from("admin-config")],
    PROGRAM_ID,
  )[0];

  // helper for getting wallet switch
  const walletTransferSwitchAddress = (wallet: PublicKey) =>
    PublicKey.findProgramAddressSync([wallet.toBuffer()], PROGRAM_ID)[0];

  // sender
  const [sender, senderTokenAccount, senderTokenAccountCreateIx] = newUser();

  it("Create Mint Account with Transfer Hook Extension", async () => {
    const extensions = [ExtensionType.TransferHook];
    const mintLen = getMintLen(extensions);
    const lamports =
      await provider.connection.getMinimumBalanceForRentExemption(mintLen);

    const transaction = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: mint.publicKey,
        space: mintLen,
        lamports: lamports,
        programId: TOKEN_2022_PROGRAM_ID,
      }),
      createInitializeTransferHookInstruction(
        mint.publicKey,
        payer.publicKey,
        program.programId, // Transfer Hook Program ID
        TOKEN_2022_PROGRAM_ID,
      ),
      createInitializeMintInstruction(
        mint.publicKey,
        decimals,
        payer.publicKey,
        null,
        TOKEN_2022_PROGRAM_ID,
      ),
    );

    transaction.recentBlockhash = svm.latestBlockhash();
    transaction.sign(payer, mint);

    svm.sendTransaction(transaction);
  });

  // Create the two token accounts for the transfer-hook enabled mint
  // Fund the sender token account with 100 tokens
  it("Create Token Accounts and Mint Tokens", async () => {
    // 100 tokens
    const amount = 100 * 10 ** decimals;

    const transaction = new Transaction().add(
      senderTokenAccountCreateIx, // create sender token account
      createMintToInstruction(
        mint.publicKey,
        senderTokenAccount,
        payer.publicKey,
        amount,
        [],
        TOKEN_2022_PROGRAM_ID,
      ),
    );

    transaction.recentBlockhash = svm.latestBlockhash();
    transaction.sign(payer);

    svm.sendTransaction(transaction);
  });

  // Account to store extra accounts required by the transfer hook instruction
  // This will be called for every mint
  //
  it("Create ExtraAccountMetaList Account", async () => {
    await program.methods
      .initializeExtraAccountMetasList()
      .accounts({
        payer: payer.publicKey,
        tokenMint: mint.publicKey,
      })
      .signers([payer])
      .rpc();
  });

  // Set the account that controls the switches for the wallet
  it("Configure an admin", async () => {
    await program.methods
      .configureAdmin()
      .accounts({
        admin: payer.publicKey,
        newAdmin: payer.publicKey,
      })
      .signers([payer])
      .rpc();

    const adminConfig =
      await program.account.adminConfig.fetch(adminConfigAddress);
    assert(adminConfig.isInitialised === true, "admin config not initialised");
    assert(
      adminConfig.admin.toBase58() === payer.publicKey.toBase58(),
      "admin does not match",
    );
  });

  // Account to store extra accounts required by the transfer hook instruction
  it("turn transfers off for sender", async () => {
    await program.methods
      .switch(false)
      .accountsPartial({
        wallet: sender.publicKey,
        admin: payer.publicKey,
      })
      .signers([payer])
      .rpc();

    const walletSwitch = await program.account.transferSwitch.fetch(
      walletTransferSwitchAddress(sender.publicKey),
    );

    assert(
      walletSwitch.wallet.toBase58() === sender.publicKey.toBase58(),
      "wallet key does not match",
    );
    assert(!walletSwitch.on, "wallet switch not set to false");
  });

  it("Try transfer, should fail!", async () => {
    // 1 tokens
    const amount = 1 * 10 ** decimals;
    const bigIntAmount = BigInt(amount);

    const [recipient, recipientTokenAccount, recipientTokenAccountCreateIx] =
      newUser();

    // create the recipient token account ahead of the transfer,
    //
    let transaction = new Transaction().add(
      recipientTokenAccountCreateIx, // create recipient token account
    );

    transaction.recentBlockhash = svm.latestBlockhash();
    transaction.sign(payer, recipient);

    svm.sendTransaction(transaction);

    // Standard token transfer instruction
    const transferInstruction =
      await createTransferCheckedWithTransferHookInstruction(
        connection,
        senderTokenAccount,
        mint.publicKey,
        recipientTokenAccount,
        sender.publicKey,
        bigIntAmount,
        decimals,
        [],
        "confirmed",
        TOKEN_2022_PROGRAM_ID,
      );

    transaction = new Transaction().add(
      transferInstruction, // transfer instruction
    );

    transaction.recentBlockhash = svm.latestBlockhash();
    transaction.sign(payer, sender);

    // expect the transaction to fail
    //
    expectRevert(svm.sendTransaction(transaction));

    const recipientTokenAccountData = (
      svm.getAccount(recipientTokenAccount)
    ).data;
    const recipientBalance = AccountLayout.decode(
      recipientTokenAccountData,
    ).amount;

    assert(recipientBalance === BigInt(0), "transfer was successful");
  });

  // Account to store extra accounts required by the transfer hook instruction
  it("turn on for sender!", async () => {
    await program.methods
      .switch(true)
      .accountsPartial({
        wallet: sender.publicKey,
        admin: payer.publicKey,
      })
      .signers([payer])
      .rpc();

    const walletSwitch = await program.account.transferSwitch.fetch(
      walletTransferSwitchAddress(sender.publicKey),
    );

    assert(
      walletSwitch.wallet.toBase58() === sender.publicKey.toBase58(),
      "wallet key does not match",
    );
    assert(walletSwitch.on, "wallet switch not set to true");
  });

  it("Send successfully", async () => {
    // 1 tokens
    const amount = 1 * 10 ** decimals;
    const bigIntAmount = BigInt(amount);

    const [_recipient, recipientTokenAccount, recipientTokenAccountCreateIx] =
      newUser();

    // Standard token transfer instruction
    const transferInstruction =
      await createTransferCheckedWithTransferHookInstruction(
        connection,
        senderTokenAccount,
        mint.publicKey,
        recipientTokenAccount,
        sender.publicKey,
        bigIntAmount,
        decimals,
        [],
        "confirmed",
        TOKEN_2022_PROGRAM_ID,
      );

    const transaction = new Transaction().add(
      recipientTokenAccountCreateIx,
      transferInstruction,
    );

    transaction.recentBlockhash = svm.latestBlockhash();
    transaction.sign(payer, sender);

    svm.sendTransaction(transaction);

    const recipientTokenAccountData = (
      svm.getAccount(recipientTokenAccount)
    ).data;

    const recipientBalance = AccountLayout.decode(
      recipientTokenAccountData,
    ).amount;

    assert(recipientBalance === bigIntAmount, "transfer was not successful");
  });
});
