import { describe, it } from "node:test";
import * as anchor from "@coral-xyz/anchor";
import {
	Keypair,
	PublicKey,
	SystemProgram,
	Transaction,  LAMPORTS_PER_SOL} from "@solana/web3.js";
import { LiteSVMProvider } from 'anchor-litesvm';
import { LiteSVM } from 'litesvm';
import type { CheckingAccountProgram } from "../target/types/checking_account_program";

import IDL from "../target/idl/checking_account_program.json" with {
	type: "json",
};
const PROGRAM_ID = new PublicKey(IDL.address);

describe("Bankrun example", async () => {
	const svm = new LiteSVM();
  svm.addProgramFromFile(PROGRAM_ID, 'target/deploy/checking_account_program.so');
  const payer = Keypair.generate();
  svm.airdrop(payer.publicKey, BigInt(100 * LAMPORTS_PER_SOL));
	const provider = new LiteSVMProvider(svm, new anchor.Wallet(payer));

	const wallet = provider.wallet as anchor.Wallet;
	const program = new anchor.Program<CheckingAccountProgram>(IDL, provider);

	// We'll create this ahead of time.
	// Our program will try to modify it.
	const accountToChange = new Keypair();
	// Our program will create this.
	const accountToCreate = new Keypair();

	it("Create an account owned by our program", async () => {
		const instruction = SystemProgram.createAccount({
			fromPubkey: provider.wallet.publicKey,
			newAccountPubkey: accountToChange.publicKey,
			lamports: await provider.connection.getMinimumBalanceForRentExemption(0),
			space: 0,
			programId: program.programId, // Our program
		});

		const transaction = new Transaction();
		const blockhash = svm.latestBlockhash();

		transaction.recentBlockhash = blockhash;
		transaction.add(instruction).sign(wallet.payer, accountToChange);
		svm.sendTransaction(transaction);
	});

	it("Check accounts", async () => {
		await program.methods
			.checkAccounts()
			.accounts({
				payer: wallet.publicKey,
				accountToCreate: accountToCreate.publicKey,
				accountToChange: accountToChange.publicKey,
			})
			.rpc();
	});
});
