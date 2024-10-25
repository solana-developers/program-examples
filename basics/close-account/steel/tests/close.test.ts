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
	CLOSE = 1,
}

describe("close account program", () => {
	let context: ProgramTestContext;
	let client: BanksClient;
	let payer: Keypair;

	before(async () => {
		context = await start(
			[{ name: "close_account_program", programId: PROGRAM_ID }],
			[],
		);
		client = context.banksClient;
		payer = context.payer;
	});

	// helpers
	const createInitializeInstructionData = (name: string): Buffer => {
		const buffer = Buffer.alloc(33);
		buffer.writeUInt8(Discriminator.INIT, 0);
		buffer.write(name.padEnd(32, "\0"), 1, "utf8"); // make sure we have 32 characters
		return buffer;
	};

	const createCloseInstructionData = (): Buffer => {
		const buffer = Buffer.alloc(1);
		buffer.writeUInt8(Discriminator.CLOSE, 0);
		return buffer;
	};

	const uint8ArrayToString = (array: Uint8Array): string => {
		return new TextDecoder().decode(array).replace(/\0+$/, "");
	};

	it("process", async () => {
		const [accountPDA] = PublicKey.findProgramAddressSync(
			[Buffer.from(ACCOUNT_SEED)],
			PROGRAM_ID,
		);

		// init
		const name = 'Alice';
		const instructionData = createInitializeInstructionData(name);

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

		console.log(`Account Data Length: ${accountInfo.data.length}`);

		const accountData = accountInfo.data.slice(8); // Skip the 8-byte discriminator
		const account_name = uint8ArrayToString(accountData.slice(0, 32));
		
		assert(account_name === name, "name is not set correctly");

		// close
		const closeInstData = createCloseInstructionData();
		const closeIx = new TransactionInstruction({
			programId: PROGRAM_ID,
			keys: [
				{ pubkey: payer.publicKey, isSigner: true, isWritable: true },
				{ pubkey: accountPDA, isSigner: false, isWritable: true },
			],
			data: closeInstData,
		});
		const closeTx = new Transaction().add(closeIx);
		closeTx.recentBlockhash = context.lastBlockhash;
		closeTx.sign(payer);

		await client.processTransaction(closeTx);
		const accountInfoAfterClose = await client.getAccount(accountPDA);
		assert(accountInfoAfterClose === null, "account should be closed");
	});
});
