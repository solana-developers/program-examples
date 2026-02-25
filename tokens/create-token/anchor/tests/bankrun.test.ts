import anchor from "@coral-xyz/anchor";

const { BN, Program, setProvider, web3 } = anchor;

import { readFileSync } from "node:fs";
import { PublicKey } from "@solana/web3.js";
import { BankrunProvider, startAnchor } from "anchor-bankrun";
import { assert } from "chai";
import type { CreateToken } from "../target/types/create_token";

const MPL_TOKEN_METADATA_PROGRAM_ID = new PublicKey(
	"metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
);

describe("Create Token with Metadata", () => {
	it("Mints and adds metadata!", async () => {
		const context = await startAnchor(
			"",
			[
				{
					name: "create_token",
					programId: new PublicKey(
						"Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS",
					),
				},
				{
					name: "mpl_token_metadata",
					programId: new PublicKey(
						"metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
					),
				},
			],
			[],
		);
		const provider = new BankrunProvider(context);
		setProvider(provider);

		const idl = JSON.parse(
			readFileSync("./target/idl/create_token.json", "utf-8"),
		);
		const program = new Program<CreateToken>(idl, provider);

		const mintKeypair = web3.Keypair.generate();

		const [metadataAddress] = PublicKey.findProgramAddressSync(
			[
				Buffer.from("metadata"),
				MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
				mintKeypair.publicKey.toBuffer(),
			],
			MPL_TOKEN_METADATA_PROGRAM_ID,
		);

		await program.methods
			.createToken("SuperToken", "SUP", "http://uri", new BN(1000))
			.accounts({
				mint: mintKeypair.publicKey,
				metadataAccount: metadataAddress,
				tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
			})
			.signers([mintKeypair])
			.rpc();

		const accountInfo = await context.banksClient.getAccount(metadataAddress);
		assert.isNotNull(accountInfo, "Metadata account should exist");

		console.log("Success: Token Minted with Metadata!");
	});
});
