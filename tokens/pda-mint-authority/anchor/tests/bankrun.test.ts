import { describe, it } from "node:test";
import * as anchor from "@coral-xyz/anchor";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { PublicKey, LAMPORTS_PER_SOL, Keypair} from "@solana/web3.js";
import { LiteSVMProvider } from 'anchor-litesvm';
import { BN } from "bn.js";
import { LiteSVM } from 'litesvm';
import type { TokenMinter } from "../target/types/token_minter";

import IDL from "../target/idl/token_minter.json";
const PROGRAM_ID = new PublicKey(IDL.address);
const METADATA_PROGRAM_ID = new PublicKey(
	"metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
);

describe("NFT Minter", async () => {
	const svm = new LiteSVM();
  svm.addProgramFromFile(PROGRAM_ID, 'target/deploy/token_minter.so');
  svm.addProgramFromFile(METADATA_PROGRAM_ID, 'target/deploy/token_metadata.so');
  const payer = Keypair.generate();
  svm.airdrop(payer.publicKey, BigInt(100 * LAMPORTS_PER_SOL));
	const provider = new LiteSVMProvider(svm, new anchor.Wallet(payer));
	anchor.setProvider(provider);
	const wallet = provider.wallet as anchor.Wallet;
	const program = new anchor.Program<TokenMinter>(IDL, provider);

	// Derive the PDA to use as mint account address.
	// This same PDA is also used as the mint authority.
	const [mintPDA] = PublicKey.findProgramAddressSync(
		[Buffer.from("mint")],
		program.programId,
	);

	const metadata = {
		name: "Solana Gold",
		symbol: "GOLDSOL",
		uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
	};

	it("Create a token!", async () => {
		const transactionSignature = await program.methods
			.createToken(metadata.name, metadata.symbol, metadata.uri)
			.accounts({
				payer: payer.publicKey,
			})
			.rpc();

		console.log("Success!");
		console.log(`   Mint Address: ${mintPDA}`);
		console.log(`   Transaction Signature: ${transactionSignature}`);
	});

	it("Mint 1 Token!", async () => {
		// Derive the associated token address account for the mint and payer.
		const associatedTokenAccountAddress = getAssociatedTokenAddressSync(
			mintPDA,
			payer.publicKey,
		);

		// Amount of tokens to mint.
		const amount = new BN(100);

		const transactionSignature = await program.methods
			.mintToken(amount)
			.accounts({
				payer: payer.publicKey,
				associatedTokenAccount: associatedTokenAccountAddress,
			})
			.rpc();

		console.log("Success!");
		console.log(
			`   Associated Token Account Address: ${associatedTokenAccountAddress}`,
		);
		console.log(`   Transaction Signature: ${transactionSignature}`);
	});
});
