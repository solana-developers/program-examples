import { describe, it } from "node:test";
import * as anchor from "@anchor-lang/core";
import { Keypair, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { BankrunProvider } from "anchor-bankrun";
import { startAnchor } from "solana-bankrun";
import IDL from "../target/idl/account_data_anchor_program.json" with { type: "json" };
import type { AccountDataAnchorProgram } from "../target/types/account_data_anchor_program";
import { assert } from "chai";

const PROGRAM_ID = new PublicKey(IDL.address);


describe("anchor-data", async () => {
	const context = await startAnchor("", [{name:"account_data_anchor_program",programId:PROGRAM_ID }], []);
	const provider = new BankrunProvider(context);
	const program = new anchor.Program<AccountDataAnchorProgram>(IDL, provider);
	
	const addressInfoAccount = new Keypair();

	await program.methods.createAddressInfo("Joe C", 136, "Mile High Dr.", "Solana Beach")
		.accounts({
			addressInfo: addressInfoAccount.publicKey,
			payer: provider.wallet.publicKey,
		})
		.signers([addressInfoAccount])
		.rpc();

		assert.ok(true)
		const addressInfo = await program.account.addressInfo.fetch(addressInfoAccount.publicKey);
		console.log(`Name     : ${addressInfo.name}`);
		console.log(`House Num: ${addressInfo.houseNumber}`);
		console.log(`Street   : ${addressInfo.street}`);
		console.log(`City     : ${addressInfo.city}`);


		it("Read the new account's data", async () => {
		let account = await program.account.addressInfo.fetch(addressInfoAccount.publicKey);
		assert.equal(account.name, "Joe C");
		assert.equal(account.houseNumber, 136);
		assert.equal(account.street, "Mile High Dr.");
		assert.equal(account.city, "Solana Beach");
		})
})