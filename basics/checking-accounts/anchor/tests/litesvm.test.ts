import assert from "node:assert/strict";
import { describe, it } from "node:test";
import {
	Keypair,
	LAMPORTS_PER_SOL,
	PublicKey,
	SystemProgram,
	Transaction,
	TransactionInstruction,
} from "@solana/web3.js";
import { LiteSVM } from "litesvm";

import IDL from "../target/idl/checking_account_program.json" with {
	type: "json",
};

describe("LiteSVM: Checking accounts", () => {
	const litesvm = new LiteSVM();

	const programId = new PublicKey(IDL.address);
	const payer = Keypair.generate();

	const programPath = new URL(
		"../target/deploy/checking_account_program.so",
		import.meta.url,
	).pathname;
	litesvm.addProgramFromFile(programId, programPath);

	litesvm.airdrop(payer.publicKey, BigInt(5 * LAMPORTS_PER_SOL));

	// We'll create this ahead of time.
	// Our program will try to modify it.
	const accountToChange = new Keypair();
	// Our program will create this.
	const accountToCreate = new Keypair();

	it("Create an account owned by our program", () => {
		const instruction = SystemProgram.createAccount({
			fromPubkey: payer.publicKey,
			newAccountPubkey: accountToChange.publicKey,
			lamports: Number(litesvm.getRent().minimumBalance(0n)),
			space: 0,
			programId: programId, // Our program
		});

		const tx = new Transaction();
		tx.recentBlockhash = litesvm.latestBlockhash();
		tx.add(instruction);
		tx.feePayer = payer.publicKey;
		tx.sign(accountToChange, payer);

		const res = litesvm.sendTransaction(tx);
		assert.equal(
			res.toString().includes("AccountNotFound"),
			false,
			"Expected account not found error",
		);
	});

	it("Check accounts", () => {
		const instruction = new TransactionInstruction({
			keys: [
				{ pubkey: payer.publicKey, isSigner: true, isWritable: false },
				{
					pubkey: accountToCreate.publicKey,
					isSigner: false,
					isWritable: true,
				},
				{
					pubkey: accountToChange.publicKey,
					isSigner: false,
					isWritable: true,
				},
				{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
			],
			programId: programId,

			data: Buffer.from(
				IDL.instructions.find((i) => i.name === "check_accounts")
					?.discriminator,
			),
		});

		const tx = new Transaction();
		tx.recentBlockhash = litesvm.latestBlockhash();
		tx.add(instruction);
		tx.feePayer = payer.publicKey;
		tx.sign(payer);

		const res = litesvm.sendTransaction(tx);
		assert.equal(
			res.toString().includes("FailedTransactionMetadata"),
			false,
			" No account found error",
		);
	});
});
