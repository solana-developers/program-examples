import * as anchor from "@coral-xyz/anchor";
import { AnchorError, Program, web3} from "@coral-xyz/anchor";
import { TokenStop } from "../target/types/token_stop";
import {
  createTransferCheckedWithTransferHookInstruction,
  ExtraAccountMeta,
  getExtraAccountMetaAddress,
} from "@solana/spl-token";

import {
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
  Keypair,
} from "@solana/web3.js";
import {
  ExtensionType,
  TOKEN_2022_PROGRAM_ID,
  getMintLen,
  createInitializeMintInstruction,
  createInitializeTransferHookInstruction,
  createMintToInstruction,
  getAccount,
  getOrCreateAssociatedTokenAccount,
  createInitializeMetadataPointerInstruction,
} from "@solana/spl-token";

import {
  createInitializeInstruction,
} from "@solana/spl-token-metadata";
import { assert } from "chai";

describe("token-stop", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TokenStop as Program<TokenStop>;
  let envProvider = anchor.AnchorProvider.env();

  const wallet = envProvider.wallet as anchor.Wallet;
  const provider = new anchor.AnchorProvider(
    new anchor.web3.Connection(envProvider.connection.rpcEndpoint, "confirmed"),
    envProvider.wallet,
    {
      skipPreflight: true,
      commitment: "confirmed"
    },
  );

  anchor.setProvider(provider);
  const connection = provider.connection;

  //const conn = new web3.Connection("https://api.mainnet-beta.solana.com");

  it("Is initialized!", async () => {
    // Add your test here.

    const sender = new Keypair();
    const receiver = new Keypair();

    // Airdrop SOL to sender and receiver
    const airdropAmount = 1000000000; // 1 SOL
    await connection.requestAirdrop(receiver.publicKey, airdropAmount);
    await connection.requestAirdrop(sender.publicKey, airdropAmount);
    
    // Create a new keypair for the mint
    const mintKeypair = Keypair.generate();

    // Calculate the length of the mint account
    const extensionTypes = [ExtensionType.TransferHook, ExtensionType.MetadataPointer];
    const mintLen = getMintLen(extensionTypes);

    // Create the mint account
    const lamports = await connection.getMinimumBalanceForRentExemption(mintLen);
    const createAccountInstruction = SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: mintKeypair.publicKey,
      space: mintLen,
      lamports,
      programId: TOKEN_2022_PROGRAM_ID,
    });

    // Initialize the mint
    const initializeMintInstruction = createInitializeMintInstruction(
      mintKeypair.publicKey,
      0, // decimals
      wallet.publicKey, // mint authority
      wallet.publicKey, // freeze authority (optional)
      TOKEN_2022_PROGRAM_ID
    );

    // Initialize the transfer hook
    const transferHookProgramId = program.programId; // Assuming your token-stop program is the transfer hook
    const initializeTransferHookInstruction = createInitializeTransferHookInstruction(
      mintKeypair.publicKey,
      wallet.publicKey, // authority
      transferHookProgramId,
      TOKEN_2022_PROGRAM_ID
    );

    // Initialize the metadata pointer
    const initializeMetadataPointerInstruction = createInitializeMetadataPointerInstruction(
      mintKeypair.publicKey,
      wallet.publicKey, // authority
      mintKeypair.publicKey, // metadata address (storing on the mint account itself)
      TOKEN_2022_PROGRAM_ID
    );

    // Initialize the metadata
    // Instruction to initialize Metadata Account data
    const initializeMetadataInstruction = createInitializeInstruction({
      programId: TOKEN_2022_PROGRAM_ID, // Token Extension Program as Metadata Program
      metadata: mintKeypair.publicKey, // Account address that holds the metadata
      updateAuthority: wallet.publicKey, // Authority that can update the metadata
      mint: mintKeypair.publicKey, // Mint Account address
      mintAuthority: wallet.publicKey, // Designated Mint Authority
      name: 'My Token',
      symbol: 'MTK',
      uri: 'https://example.com/token-metadata',
    });

    // Create and send the transaction
    const tx = new Transaction().add(
      createAccountInstruction,
      initializeTransferHookInstruction,
      initializeMetadataPointerInstruction,
      initializeMintInstruction,
      //initializeMetadataInstruction,
    );

    await sendAndConfirmTransaction(connection, tx, [wallet.payer, mintKeypair]);

    console.log("Mint created with transfer hook and metadata extensions:", mintKeypair.publicKey.toBase58());

    // Send more lamports to mintKeypair
    const transferInstruction2 = SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      toPubkey: mintKeypair.publicKey,
      lamports: 1000000000, // 1 SOL, adjust as needed
    });

    // Create and send the transaction
    const tx2 = new Transaction().add(
      transferInstruction2,
      initializeMetadataInstruction,
    );

    await program.methods
      .initialize()
      .accounts({ mint: mintKeypair.publicKey, mintAuthority: wallet.publicKey})
      .rpc();

    await sendAndConfirmTransaction(connection, tx2, [wallet.payer, mintKeypair]);

    console.log("metadata created with transfer hook and metadata extensions:", mintKeypair.publicKey.toBase58());

    // Create sender's token account
    const senderTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      sender,
      mintKeypair.publicKey,
      sender.publicKey,
      false,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    // Mint tokens to sender
    const mintAmount = 1000000000; // Adjust as needed
    const mintToInstruction = createMintToInstruction(
      mintKeypair.publicKey,
      senderTokenAccount.address,
      wallet.publicKey,
      mintAmount,
      [],
      TOKEN_2022_PROGRAM_ID
    );

    const mintTx = new Transaction().add(mintToInstruction);
    await sendAndConfirmTransaction(connection, mintTx, [wallet.payer]);

    console.log(`Minted ${mintAmount} tokens to sender's account:`, senderTokenAccount.address.toBase58());

    // Call the program to stop transfers
    await program.methods
      .stopTransfer()
      .accounts({
        authority: wallet.publicKey,
        mint: mintKeypair.publicKey,
      })
      .rpc();

    console.log("Transfers stopped");

    // Create receiver's token account
    const receiverTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      receiver,
      mintKeypair.publicKey,
      receiver.publicKey,
      false,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    // Attempt to transfer tokens from sender to receiver
    const transferAmount = 100000000; // 0.1 tokens
    const transferInstruction = await createTransferCheckedWithTransferHookInstruction(
      connection,
      senderTokenAccount.address,
      mintKeypair.publicKey,
      receiverTokenAccount.address,
      sender.publicKey,
      BigInt(transferAmount),
      0, // decimals
      [],
      "confirmed",
      TOKEN_2022_PROGRAM_ID
    );

    // Add extra accounts required by the transfer hook
    const transferTx = new Transaction().add(transferInstruction);

    try {
      await sendAndConfirmTransaction(connection, transferTx, [sender], {
        skipPreflight: true, commitment: "confirmed"
      });
      console.log("Transfer succeeded (unexpected)");
      
      throw new Error("Transfer should have failed when transfers are stopped");
    } catch (error) {
      console.log("Transfer failed as expected:");
      
      if (error instanceof anchor.web3.SendTransactionError) {
        console.log("SendTransactionError caught:");
        console.log("Error message:", error.message);
        console.log("Error logs:", await error.getLogs(connection));
        //console.log("Error logs:", error.logs);
        
        // Try to parse the error as an AnchorError
        const anchorError = AnchorError.parse(error.logs);
        if (anchorError) {
          console.log("Parsed AnchorError:");
          console.log("Program:", anchorError.program);
          console.log("Error code:", anchorError.error.errorCode.code);
          console.log("Error message:", anchorError.error.errorMessage);
        }
      } else if (error instanceof AnchorError) {
        console.log("AnchorError caught:");
        console.log("Program:", error.program);
        console.log("Error code:", error.error.errorCode.code);
        console.log("Error message:", error.error.errorMessage);
      } else {
        console.log("Unexpected error type:", error);
      }

      // Assert that the error is either a SendTransactionError or an AnchorError
      assert.isTrue(
        error instanceof anchor.web3.SendTransactionError || 
        error instanceof AnchorError,
        "Expected SendTransactionError or AnchorError"
      );
    }

    // Re-enable transfers
    await program.methods
      .resumeTransfer()
      .accounts({
        authority: wallet.publicKey,
        mint: mintKeypair.publicKey,
      })
      .rpc();

    console.log("Transfers re-enabled");

    const transferInstruction3 = await createTransferCheckedWithTransferHookInstruction(
      connection,
      senderTokenAccount.address,
      mintKeypair.publicKey,
      receiverTokenAccount.address,
      sender.publicKey,
      BigInt(10000),
      0, // decimals
      [],
      "confirmed",
      TOKEN_2022_PROGRAM_ID
    );
    // Attempt to transfer tokens again
    const secondTransferTx = new Transaction().add(transferInstruction3);

    try {
      await sendAndConfirmTransaction(connection, secondTransferTx, [sender], {
        skipPreflight: true,
      });
      console.log("Second transfer succeeded (expected)");
      
      // Verify the transfer
      const updatedReceiverAccount = await getAccount(connection, receiverTokenAccount.address, undefined, TOKEN_2022_PROGRAM_ID);
      console.log("Receiver's balance after transfer:", updatedReceiverAccount.amount.toString());
    } catch (error) {
      console.log("Second transfer failed (unexpected):", error.message);
      throw new Error("Second transfer should have succeeded");
    }
  });
});