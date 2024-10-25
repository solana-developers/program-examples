import * as anchor from "@coral-xyz/anchor";
import { CloseAccount } from "../target/types/close_account";
import { before } from "mocha";
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { assert } from "chai";

const IDL = require('../target/idl/close_account.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe("close-account", async () => {
    // Configure the client to use the local cluster.
    const context = await startAnchor('', [{ name: 'close_account', programId: PROGRAM_ID }], []);
    const provider = new BankrunProvider(context);
  
    const program = new anchor.Program<CloseAccount>(IDL, provider);
    const user = Keypair.generate(); // Generate a new user keypair
  
    // Variables that will store the user account PDA and its bump
    let userAccount: PublicKey;
    let userAccountBump: number;
    
    before(async () => {
      const latestBlockHash = await provider.connection.getLatestBlockhash();
  
      // Airdrop 1 SOL to the generated wallet for testing the transactions
      const airdropUser = await provider.connection.requestAirdrop(user.publicKey, 1 * LAMPORTS_PER_SOL);
  
      await provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: airdropUser,
      });
  
      // Derive PDA for the user account
      [userAccount, userAccountBump] = await PublicKey.findProgramAddressSync(
        [
          Buffer.from("user"),
          user.publicKey.toBuffer(),
        ], program.programId);
    });
  
    it("Create User Account", async () => {
      // Create User Account instruction invoked from the program
      await program.methods
      .createUserAccount()
      .accountsPartial({
        user: user.publicKey, // User's public key
        userAccount, // PDA for the user account
      })
      .signers([user]) // Sign the transaction with the user's keypair
      .rpc();
  
      // Fetch and assert the accounts data
      const userAccountData = await program.account.accountState.fetch(userAccount);
      assert.equal(userAccountData.user.toBase58(), user.publicKey.toBase58()); // Verify the user account data
    });
  
    it("Close User Account", async () => {
      // Close User Account instruction invoked from the program
      await program.methods
      .closeUserAccount()
      .accountsPartial({
        user: user.publicKey, // User's public key
        userAccount, // PDA for the user account
      })
      .signers([user]) // Sign the transaction with the user's keypair
      .rpc();

    // The account should no longer exist, returning null.
    try {
        const userAccountData = await program.account.accountState.fetchNullable(userAccount);
        assert.equal(userAccountData, null); // Verify the user account is closed
      } catch (err) {
        // Won't return null and will throw an error in anchor-bankrun'
        assert.equal(err.message, `Could not find ${userAccount}`);
      }
    }); 
  });



