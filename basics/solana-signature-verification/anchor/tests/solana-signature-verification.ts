import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  PythSolanaReceiver,
  InstructionWithEphemeralSigners,
} from "@pythnetwork/pyth-solana-receiver";
import { Ed25519Program, Connection } from "@solana/web3.js";
import { SolanaSignatureVerification } from "../target/types/solana_signature_verification";
import * as ed from '@noble/ed25519';
import * as fs from 'fs';
import * as path from 'path';
import { getPriceUpdateData, transferLamports } from "./utils"
import { assert, expect } from 'chai';
import * as crypto from 'crypto';

const MSG = crypto.randomBytes(32);
// replace with your keypair path
const keypairPath = "~/.config/solana/id.json";
// the actual keypair
const payer = anchor.web3.Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync(path.resolve(keypairPath), 'utf-8'))));
// according to contract there can be one escrow for one user so we will create a temporary one for testing
const temp_payer = anchor.web3.Keypair.generate();
let signature: Uint8Array;
const HERMES_URL = "https://hermes.pyth.network/";
// get desired feed
// https://www.pyth.network/developers/price-feed-ids
const SOL_PRICE_FEED_ID =
  "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
const connection = new Connection("https://api.devnet.solana.com", "confirmed");
let pythSolanaReceiver: PythSolanaReceiver;
let priceUpdateData: string[];
let priceUpdateAccount: string
const shardId = 1;
describe("solana-signature-verification", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolanaSignatureVerification as Program<SolanaSignatureVerification>;
  before(async () => {

    // Generate ed25519 signature
    signature = await ed.sign(MSG, temp_payer.secretKey.slice(0, 32));
    // Verify signature locally before sending to chain
    const isValid = await ed.verify(signature, MSG, temp_payer.publicKey.toBytes());
    expect(isValid).to.be.true;
    pythSolanaReceiver = new PythSolanaReceiver({
      connection: connection,
      wallet: new anchor.Wallet(temp_payer),
    });
    priceUpdateData = await getPriceUpdateData(SOL_PRICE_FEED_ID);
    console.log(`Posting price update: ${priceUpdateData}`);
    priceUpdateAccount = pythSolanaReceiver
      .getPriceFeedAccountAddress(shardId, SOL_PRICE_FEED_ID)
      .toBase58()
    console.log("price updates will be posted to", priceUpdateAccount);
  });
  it("Deposit to Escrow", async () => {
    // give some lamports to temporary keypair so that he can do an escrow
    await transferLamports(provider.connection, payer, temp_payer.publicKey, 0.2);
    try {
      const [escrowState] = await anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("instruction-introspection-seed"), temp_payer.publicKey.toBuffer()],
        program.programId,
      );
      const amountToLockUp = new anchor.BN(100);
      console.log("Amount to lock", amountToLockUp.toString());
      const unlockPrice = new anchor.BN(200);
      console.log("Unlock price", unlockPrice.toString());


      console.log("Escrow state (public key):", escrowState.toBase58()); // Debugging line
      console.log("Creating escrow account...");
      const tx = new anchor.web3.Transaction();

      tx.add(
        Ed25519Program.createInstructionWithPublicKey({
          publicKey: temp_payer.publicKey.toBytes(),
          message: MSG,
          signature: signature,
        })
      );
      tx.add(
        await program.methods.deposit(amountToLockUp,
          unlockPrice)
          .accounts({
            user: temp_payer.publicKey,
            escrowAccount: escrowState,
            systemProgram: anchor.web3.SystemProgram.programId,
            instructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
          })
          .signers([temp_payer])
          .instruction()
      );
      // Re - fetch the latest blockhash to ensure it's still valid
      let blockhash = (await provider.connection.getLatestBlockhash('finalized')).blockhash;
      let blockheight = await provider.connection.getBlockHeight("confirmed");
      // Set the new blockhash in the transaction
      tx.recentBlockhash = blockhash;
      tx.lastValidBlockHeight = blockheight;
      tx.sign(temp_payer); // imp:always sign after getting block hash
      const txid = await provider.connection.sendRawTransaction(tx.serialize());
      await provider.connection.confirmTransaction(
        txid
      );
      console.log("Escrow created successfully. TX:", txid);
      const escrowAccount = await program.account.escrowState.fetch(escrowState);

      const escrowBalance = await provider.connection.getBalance(escrowState, "confirmed");
      assert(escrowBalance > 0)
    } catch (error) {
      console.error("Error details:", error.logs);
      throw new Error(`Failed to create escrow: ${error.message}`);
    }


  });
  it("withdraws from escrow", async () => {

    const [escrowState] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("instruction-introspection-seed"), temp_payer.publicKey.toBuffer()],
      program.programId,
    );
    const userBalanceBefore = await provider.connection.getBalance(
      temp_payer.publicKey
    );

    try {
      const transactionBuilder = pythSolanaReceiver.newTransactionBuilder({});
      await transactionBuilder.addUpdatePriceFeed(priceUpdateData, shardId);
      const ix: InstructionWithEphemeralSigners[] = [
        {
          instruction: Ed25519Program.createInstructionWithPublicKey({
            publicKey: temp_payer.publicKey.toBytes(),
            message: MSG,
            signature: signature,
          }),
          signers: []

        },
        {
          instruction: await program.methods
            .withdraw(SOL_PRICE_FEED_ID) // Replace with your actual method and parameters
            .accounts({
              user: temp_payer.publicKey,
              escrowAccount: escrowState,
              priceUpdate: priceUpdateAccount,
              systemProgram: anchor.web3.SystemProgram.programId,
              instructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,

            }).signers([temp_payer])
            .instruction(),
          signers: [temp_payer],
          computeUnits: 1000000

        }

      ];
      transactionBuilder.addInstruction(ix[0]);
      transactionBuilder.addInstruction(ix[1]);
      // way-one
      const txs = await pythSolanaReceiver.provider.sendAll(
        await transactionBuilder.buildVersionedTransactions({
          computeUnitPriceMicroLamports: 50000,
          tightComputeBudget: true,
        }),
        { skipPreflight: true }
      );
      try {
        //  Verify escrow account is closed
        await program.account.escrowState.fetch(escrowState);
        assert.fail("Escrow account should have been closed");
      } catch (error) {
        console.log(error.message);
        assert(
          error.message.includes("Account does not exist"),
          "Unexpected error: " + error.message
        );
      }
      // Verify user balance increased
      const userBalanceAfter = await provider.connection.getBalance(
        temp_payer.publicKey
      );
      assert(
        userBalanceAfter > userBalanceBefore,
        "User balance should have increased"
      );
      // transfer money back to original keypair
      const balance = await provider.connection.getBalance(temp_payer.publicKey);
      console.log(`Original Balance: ${balance} lamports`);
      await transferLamports(provider.connection, temp_payer, payer.publicKey, 0.1);

    } catch (error) {
      console.error("Error details:", error.transactionMessage);
      throw new Error(`Failed to withdraw from escrow: ${error.message}`);
    }
  });

});

