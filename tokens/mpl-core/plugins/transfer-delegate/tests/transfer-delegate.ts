import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TransferDelegate  } from "../target/types/transfer_delegate";

describe("transfer-delegate", () => {
	// configure anchor to use devnet cluster and  ~/.config/solana/id.json as signer
	anchor.setProvider(anchor.AnchorProvider.env());
	const provider = anchor.getProvider();

	const program = anchor.workspace
		.TransferDelegate as Program<TransferDelegate>;

	it("Set a Transfer Delegate for my asset!", async () => {
		const txHash = await program.methods
			.setTransferDelegate()
			.accounts({
				signer: provider.publicKey,
				delegate: new anchor.web3.PublicKey(
					"J63YroB8AwjDVjKuxjcYFKypVM3aBeQrfrVmNBxfmThB"
				),
				systemProgram: anchor.web3.SystemProgram.programId,
				asset: new anchor.web3.PublicKey(
					"F6sijQLDghLeyXzLBUAfDofpmizTVNVbvjp6ES7vAjbC"
				),
				coreProgram: new anchor.web3.PublicKey(
					"CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
				),
			})
			.rpc();

		console.log(
			`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet`
		);
	});
});
