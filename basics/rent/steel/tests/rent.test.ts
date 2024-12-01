import * as borsh from "@coral-xyz/borsh";
import {
	type Keypair,
	PublicKey,
	SystemProgram,
	Transaction,
	TransactionInstruction,
} from "@solana/web3.js";
import { assert } from "chai";
import { before, describe, it } from "mocha";
import {
	type BanksClient,
	type ProgramTestContext,
	start,
} from "solana-bankrun";

// Constants
const PROGRAM_ID = new PublicKey(
	"FFJjpuXzZeBM8k1aTzzUrV9tgboUWtAaKH6U2QudoH2K",
);
const ACCOUNT_SEED = "account";

enum Discriminator {
	INIT = 0,
}

interface InitInstructionData {
	variant: Discriminator;
	name: Buffer;
	address: Buffer;
}

const INIT_INSTRUCTION_LAYOUT = borsh.struct<InitInstructionData>([
	borsh.u8("variant"),
	borsh.array(borsh.u8(), 32, "name"),
	borsh.array(borsh.u8(), 64, "address"),
]);

describe("rent program", () => {
	let context: ProgramTestContext;
	let client: BanksClient;
	let payer: Keypair;

	before(async () => {
		context = await start(
			[{ name: "rent_program", programId: PROGRAM_ID }],
			[],
		);
		client = context.banksClient;
		payer = context.payer;
	});

	const createInitializeInstructionData = (
		name: string,
		address: string,
	): Buffer => {
		const buffer = Buffer.alloc(97); // 1 byte for variant + 32 bytes for name + 64 bytes for address
		INIT_INSTRUCTION_LAYOUT.encode(
			{
				variant: Discriminator.INIT,
				name: Buffer.from(name.padEnd(32, "\0").slice(0, 32)),
				address: Buffer.from(address.padEnd(64, "\0").slice(0, 64)),
			},
			buffer,
		);
		return buffer;
	};

	const uint8ArrayToString = (array: Uint8Array): string => {
		return new TextDecoder().decode(array).replace(/\0+$/, "");
	};

	it("initialize account", async () => {
		const [accountPDA] = PublicKey.findProgramAddressSync(
			[Buffer.from(ACCOUNT_SEED)],
			PROGRAM_ID,
		);

		const name = "John Doe";
		const address = "123 Main St, Anytown, USA";
		const instructionData = createInitializeInstructionData(name, address);

		console.log("Buffer length:", instructionData.length);

		const initializeIx = new TransactionInstruction({
			programId: PROGRAM_ID,
			keys: [
				{ pubkey: payer.publicKey, isSigner: true, isWritable: true },
				{ pubkey: accountPDA, isSigner: false, isWritable: true },
				{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
			],
			data: instructionData,
		});

		const initializeTx = new Transaction().add(initializeIx);
		initializeTx.recentBlockhash = context.lastBlockhash;
		initializeTx.sign(payer);

		await client.processTransaction(initializeTx);

		const accountInfo = await client.getAccount(accountPDA);
		assert(accountInfo !== null, "account should exist");

		const data = accountInfo.data.slice(8); // Skip the 8-byte discriminator

		const account_name = uint8ArrayToString(data.slice(0, 32));
		const account_address = uint8ArrayToString(data.slice(32, 96));

		console.log("TEST RESULTS:");
		console.log("  Name:", account_name);
		console.log("  Address:", account_address);

		assert.strictEqual(account_name, name, "name is not set correctly!");
	});
});
