import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { TransferHook } from "../target/types/transfer_hook";

describe("transfer_hook", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TransferHook as Program<TransferHook>;

  let walletState: PublicKey;
  let extraAccountMetaList: PublicKey;
  const mint = new PublicKey("YourMintAddressHere"); // Replace with your Mint address

  before(async () => {
    // Derive the PDA for wallet state and extra account metadata list
    [walletState] = await PublicKey.findProgramAddress(
      [Buffer.from("wallet_state")],
      program.programId
    );

    [extraAccountMetaList] = await PublicKey.findProgramAddress(
      [Buffer.from("extra-account-metas"), mint.toBuffer()],
      program.programId
    );
  });

  it("Initializes the extra account meta list and wallet state", async () => {
    await program.methods
      .initializeExtraAccountMetaList()
      .accounts({
        payer: provider.wallet.publicKey,
      })
      .signers([])
      .rpc();

    const walletStateAccount = await program.account.walletState.fetch(walletState);
    assert.isTrue(walletStateAccount.isTransferEnabled, "Transfer should be enabled by default");
  });

  it("Disables transfers for the wallet", async () => {
    await program.methods
      .toggleWalletPermission()
      .accounts({
        walletState,
      })
      .rpc();

    const walletStateAccount = await program.account.walletState.fetch(walletState);
    assert.isFalse(walletStateAccount.isTransferEnabled, "Transfer should be disabled");
  });

  it("Fails transfer when transfer permission is disabled", async () => {
    try {
      await program.methods
        .transferHook(new anchor.BN(1000)) // Amount is just an example
        .accounts({
          sourceToken: provider.wallet.publicKey, // Replace with actual token account
          destinationToken: new PublicKey("DestinationTokenAccountHere"), // Replace with actual destination
        })
        .rpc();
      assert.fail("Expected transfer to fail, but it succeeded");
    } catch (error) {
      assert.isTrue(error.message.includes("Transfer not allowed"), "Expected transfer failure due to disabled permission");
    }
  });

  it("Enables transfers for the wallet", async () => {
    await program.methods
      .toggleWalletPermission()
      .accounts({
        walletState,
      })
      .rpc();

    const walletStateAccount = await program.account.walletState.fetch(walletState);
    assert.isTrue(walletStateAccount.isTransferEnabled, "Transfer should be enabled");
  });

  it("Succeeds transfer when transfer permission is enabled", async () => {
    try {
      await program.methods
        .transferHook(new anchor.BN(1000)) // Amount is just an example
        .accounts({
          sourceToken: provider.wallet.publicKey, // Replace with actual token account
          destinationToken: new PublicKey("DestinationTokenAccountHere"), // Replace with actual destination
        })
        .rpc();
      console.log("Transfer succeeded with enabled permission");
    } catch (error) {
      assert.fail("Expected transfer to succeed, but it failed");
    }
  });
});