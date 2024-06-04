import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MintAsset } from "../target/types/mint_asset";

describe("mint-asset", () => {
	// configure anchor to use devnet cluster and  ~/.config/solana/id.json as signer
	anchor.setProvider(anchor.AnchorProvider.env());
	const provider = anchor.getProvider();

	const program = anchor.workspace.MintAsset as Program<MintAsset>;

	const metadata = {
		name: "Kobeni Supremacy",
		uri: "https://raw.githubusercontent.com/687c/mint-core-asset/main/metadata.json",
	};

	it("Is Mints a Core Asset!", async () => {
		const asset = anchor.web3.Keypair.generate();

		const txHash = await program.methods
			.mintAsset(metadata.name, metadata.uri)
			.accounts({
				signer: provider.publicKey,
				systemProgram: anchor.web3.SystemProgram.programId,
				asset: asset.publicKey,
				coreProgram: new anchor.web3.PublicKey(
					"CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
				),
			})
			.signers([asset])
			.rpc();

		console.log(
			`mint Address: https://explorer.solana.com/address/${asset.publicKey}?cluster=devnet\n`
		);
		console.log(
			`mint tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet`
		);
	});
});
