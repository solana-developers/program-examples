import type { Program } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";
import type { CloseAccountProgram } from "../target/types/close_account_program.ts";

describe("Anchor: Close an account", () => {
	/**
	 * Configure the client to use the local cluster.
	 */
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const program = anchor.workspace
		.CloseAccountProgram as Program<CloseAccountProgram>;
	const payer = provider.wallet as anchor.Wallet;

	/**
	 * Derive the PDA for the user's account.
	 */
	const [userAccountAddress] = PublicKey.findProgramAddressSync(
		[Buffer.from("USER"), payer.publicKey.toBuffer()],
		program.programId,
	);

	it("Create an account", async () => {
		await program.methods
			.createUser("John Doe")
			.accounts({
				user: payer.publicKey,
				userAccount: userAccountAddress,
				system_program: SystemProgram.programId,
			})
			.rpc();

		/**
		 * Fetch account
		 */
		const userAccount =
			await program.account.userState.fetch(userAccountAddress);
		assert.equal(userAccount.name, "John Doe");
		assert.equal(userAccount.user.toBase58(), payer.publicKey.toBase58());
	});

	it("Close an account", async () => {
		await program.methods
			.closeUser()
			.accounts({
				user: payer.publicKey,
				userAccount: userAccountAddress,
			})
			.rpc();

		/**
		 * Fetch account
		 */
		const userAccount =
			await program.account.userState.fetchNullable(userAccountAddress);
		assert.equal(userAccount, null);
	});
});
