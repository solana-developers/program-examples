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

describe("account data program", () => {
	let context: ProgramTestContext;
	let client: BanksClient;
	let payer: Keypair;

	before(async () => {
		context = await start(
			[{ name: "account_data_program", programId: PROGRAM_ID }],
			[],
		);
		client = context.banksClient;
		payer = context.payer;
	});

	// helpers
	const createInitializeInstructionData = (
		name: string,
		house_number: number,
		city: string,
		street: string,
	): Buffer => {
		const buffer = Buffer.alloc(1 + 64 + 1 + 64 + 64);
		buffer.writeUInt8(Discriminator.INIT, 0);
		buffer.write(name.padEnd(64, "\0"), 1, "utf8"); // make sure we have 64 characters
		buffer.writeUInt8(house_number, 65);
		buffer.write(city.padEnd(64, "\0"), 66, "utf8");
		buffer.write(street.padEnd(64, "\0"), 130, "utf8");
		return buffer;
	};

	const uint8ArrayToString = (array: Uint8Array): string => {
		return new TextDecoder().decode(array).replace(/\0+$/, "");
	};

	it("initialize account data", async () => {
		const [accountPDA] = PublicKey.findProgramAddressSync(
			[Buffer.from(ACCOUNT_SEED)],
			PROGRAM_ID,
		);

		const name = "John Doe";
		const house_number = 3;
		const city = "Anytown";
		const street = "Main St";

		const instructionData = createInitializeInstructionData(
			name,
			house_number,
			city,
			street,
		);
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

		const accountData = accountInfo.data.slice(8); // Skip the 8-byte discriminator
		const account_name = uint8ArrayToString(accountData.slice(0, 64));
		const account_house_number = accountData[64];
		const account_street = uint8ArrayToString(accountData.slice(65, 129));
		const account_city = uint8ArrayToString(accountData.slice(129, 193));

		assert(account_name === name, "name is not set correctly");
		assert(
			account_house_number === house_number,
			"house_number is not set correctly",
		);
		assert(account_city === city, "city is not set correctly");
		assert(account_street === street, "street is not set correctly");
	});
});
