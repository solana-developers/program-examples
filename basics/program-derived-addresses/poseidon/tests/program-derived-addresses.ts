import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import type { ProgramDerivedAddresses } from "../target/types/program_derived_addresses";

describe("PDAs", () => {
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);
	const payer = provider.wallet as anchor.Wallet;
	const program = anchor.workspace
		.ProgramDerivedAddresses as anchor.Program<ProgramDerivedAddresses>;

	// PDA for the page visits account
	const [pageVisitPDA] = PublicKey.findProgramAddressSync(
		[Buffer.from("page_visits"), payer.publicKey.toBuffer()],
		program.programId
	);

	it("Create the page visits tracking PDA", async () => {
		await program.methods
			.createPageVisits()
			.accounts({
				user: payer.publicKey,
			})
			.rpc();
	});

	it("Visit the page!", async () => {
		await program.methods
			.incrementPageVisits()
			.accounts({
				pageVisits: pageVisitPDA,
			})
			.rpc();
	});

	it("Visit the page!", async () => {
		await program.methods
			.incrementPageVisits()
			.accounts({
				pageVisits: pageVisitPDA,
			})
			.rpc();
	});

	it("View page visits", async () => {
		const pageVisits = await program.account.pageVisits.fetch(pageVisitPDA);
		console.log(`Number of page visits: ${pageVisits.pageVisits}`);
	});
});
