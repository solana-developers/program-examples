import { Buffer } from "node:buffer";
import { describe, test } from "node:test";
import {
	Keypair,
	PublicKey,
	SystemProgram,
	Transaction,
	TransactionInstruction,  LAMPORTS_PER_SOL} from "@solana/web3.js";
import * as borsh from "borsh";
import { LiteSVM } from 'litesvm';

class Assignable {
	constructor(properties) {
		for (const [key, value] of Object.entries(properties)) {
			this[key] = value;
		}
	}
}

class AddressInfo extends Assignable {
	street: string;
	city: string;
	name: string;
	house_number: number;
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

describe("Account Data!", async () => {
	const addressInfoAccount = Keypair.generate();
	const PROGRAM_ID = PublicKey.unique();
	const svm = new LiteSVM();
  svm.addProgramFromFile(PROGRAM_ID, 'tests/fixtures/account_data_native_program.so');
	

	test("Create the address info account", async () => {
		const payer = Keypair.generate();
  svm.airdrop(payer.publicKey, BigInt(10 * LAMPORTS_PER_SOL));

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

		const blockhash = svm.latestBlockhash();

		const tx = new Transaction();
		tx.recentBlockhash = blockhash;
		tx.add(ix).sign(payer, addressInfoAccount);
		svm.sendTransaction(tx);
	});

	test("Read the new account's data", async () => {
		const accountInfo = svm.getAccount(addressInfoAccount.publicKey);

		const readAddressInfo = AddressInfo.fromBuffer(
			Buffer.from(accountInfo.data),
		);
		console.log(`Name     : ${readAddressInfo.name}`);
		console.log(`House Num: ${readAddressInfo.house_number}`);
		console.log(`Street   : ${readAddressInfo.street}`);
		console.log(`City     : ${readAddressInfo.city}`);
	});
});

