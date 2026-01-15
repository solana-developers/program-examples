import { Buffer } from "node:buffer";
import { readFileSync } from "node:fs";
import { describe, test } from "node:test";
import {
	Keypair,
	PublicKey,
	SystemProgram,
	Transaction,
	TransactionInstruction,
} from "@solana/web3.js";
import * as borsh from "borsh";
import { LiteSVM } from "litesvm";

class Assignable {
	constructor(properties) {
		for (const [key, value] of Object.entries(properties)) {
			this[key] = value;
		}
	}
}

class AddressInfo extends Assignable {
	toBuffer() {
		return Buffer.from(borsh.serialize(AddressInfoSchema, this));
	}

	static fromBuffer(buffer: Buffer) {
		return borsh.deserialize(AddressInfoSchema, AddressInfo, buffer);
	}
}
const AddressInfoSchema = new Map([
	[
		AddressInfo,
		{
			kind: "struct",
			fields: [
				["name", "string"],
				["house_number", "u8"],
				["street", "string"],
				["city", "string"],
			],
		},
	],
]);

describe("Account Data!", () => {
	const addressInfoAccount = Keypair.generate();

	// Load the program keypair
	const programKeypairPath = new URL(
		"./fixtures/account_data_native_program-keypair.json",
		// @ts-ignore
		import.meta.url,
	).pathname;
	const programKeypairData = JSON.parse(readFileSync(programKeypairPath, "utf-8"));
	const programKeypair = Keypair.fromSecretKey(new Uint8Array(programKeypairData));
	const PROGRAM_ID = programKeypair.publicKey;

	const litesvm = new LiteSVM();
	const payer = Keypair.generate();

	// Load the program
	const programPath = new URL(
		"./fixtures/account_data_native_program.so",
		// @ts-ignore
		import.meta.url,
	).pathname;
	litesvm.addProgramFromFile(PROGRAM_ID, programPath);

	// Fund the payer account
	litesvm.airdrop(payer.publicKey, BigInt(100000000000));

	test("Create the address info account", () => {
		console.log(`Program Address      : ${PROGRAM_ID}`);
		console.log(`Payer Address      : ${payer.publicKey}`);
		console.log(`Address Info Acct  : ${addressInfoAccount.publicKey}`);

		const ix = new TransactionInstruction({
			keys: [
				{
					pubkey: addressInfoAccount.publicKey,
					isSigner: true,
					isWritable: true,
				},
				{ pubkey: payer.publicKey, isSigner: true, isWritable: true },
				{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
			],
			programId: PROGRAM_ID,
			data: new AddressInfo({
				name: "Joe C",
				house_number: 136,
				street: "Mile High Dr.",
				city: "Solana Beach",
			}).toBuffer(),
		});

		const tx = new Transaction().add(ix);
		tx.feePayer = payer.publicKey;
		tx.recentBlockhash = litesvm.latestBlockhash();
		tx.sign(payer, addressInfoAccount);
		litesvm.sendTransaction(tx);
	});

	test("Read the new account's data", () => {
		const accountInfo = litesvm.getAccount(addressInfoAccount.publicKey);

		if (!accountInfo) {
			throw new Error("Failed to fetch account info");
		}

		const readAddressInfo = AddressInfo.fromBuffer(
			Buffer.from(accountInfo.data),
		);
		console.log(`Name     : ${readAddressInfo.name}`);
		console.log(`House Num: ${readAddressInfo.house_number}`);
		console.log(`Street   : ${readAddressInfo.street}`);
		console.log(`City     : ${readAddressInfo.city}`);
	});
});

