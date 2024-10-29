import assert from 'node:assert';
import {before, describe, it} from "node:test"
import * as anchor from '@coral-xyz/anchor';
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  TransactionMessage,
  VersionedTransaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { BanksClient, BanksTransactionResultWithMeta, startAnchor } from 'solana-bankrun';
import type { CloseAccount } from '../target/types/close_account';

const IDL = require('../target/idl/close_account');
const PROGRAM_ID = new PublicKey(IDL.address);

async function createAndProcessTransaction(
  client: BanksClient,
  payer: Keypair,
  instruction: TransactionInstruction,
  additionalSigners: Keypair[] = [],
): Promise<BanksTransactionResultWithMeta> {
  const tx = new Transaction();
  // Get the latest blockhash
  const [latestBlockhash] = await client.getLatestBlockhash();
  tx.recentBlockhash = latestBlockhash;
  // Add transaction instructions
  tx.add(instruction);
  tx.feePayer = payer.publicKey;
  //Add signers
  tx.sign(payer, ...additionalSigners);
  // Process transaction
  const result = await client.tryProcessTransaction(tx);
  return result;
}

describe('Close an account', async () => {
  // Configure the client to use the local cluster.
  const context = await startAnchor(
    "",
    [{ name: "close_account_program", programId: PROGRAM_ID }],
    []
  );
  const provider = new BankrunProvider(context);

  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<CloseAccount>(IDL, provider);

  const user = Keypair.generate(); // Generate a new user keypair

  before(async () => {
    //Transfer SOL to the user account to cover rent
    const transferInstruction = SystemProgram.transfer({
      fromPubkey: payer.publicKey,
      toPubkey: user.publicKey,
      lamports: 2 * LAMPORTS_PER_SOL,
    });

    await createAndProcessTransaction(
      context.banksClient,
      payer.payer,
      transferInstruction,
      [payer.payer]
    );
  });

  // Derive the PDA for the user's account.
  const [userAccount, userAccountBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("USER"), user.publicKey.toBuffer()],
    program.programId
  );

  it("Can create an account", async () => {
    await program.methods
      .createUser("Jacob")
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();

    // Fetch the account data
    const userAccountData = await program.account.userState.fetch(userAccount);
    assert.equal(userAccountData.name, "Jacob");
    assert.equal(userAccountData.user.toBase58(), user.publicKey.toBase58());
    assert.notEqual(userAccountData, null);
  });

  it("Can close an Account", async () => {
    await program.methods
      .closeUser()
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();

    // The account should no longer exist, returning null.
    try {
      const userAccountData = await program.account.userState.fetchNullable(
        userAccount
      );
      assert.equal(userAccountData, null);
    } catch (err) {
      // Won't return null and will throw an error in anchor-bankrun'
      assert.equal(err.message, `Could not find ${userAccount}`);
    }
  });
});
