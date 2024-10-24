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

describe("checking account program", () => {
	let context: ProgramTestContext;
	let client: BanksClient;
	let payer: Keypair;

	before(async () => {
		context = await start(
			[{ name: "checking_account_program", programId: PROGRAM_ID }],
			[],
		);
		client = context.banksClient;
		payer = context.payer;
	});

	// helpers
	const createInitializeInstructionData = (): Buffer => {
		const buffer = Buffer.alloc(1);
		buffer.writeUInt8(Discriminator.INIT, 0);
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

		const instructionData = createInitializeInstructionData();

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
	});
});
