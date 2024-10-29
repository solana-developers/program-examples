import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { TransferHookExample } from "../target/types/transfer_hook";

describe("transfer-hook-example", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .TransferHookExample as Program<TransferHookExample>;

  let walletState = anchor.web3.Keypair.generate();
  let walletOwner = provider.wallet as anchor.Wallet;

  before(async () => {
    const lamports = await provider.connection.getMinimumBalanceForRentExemption(8);

    const tx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: walletOwner.publicKey,
        newAccountPubkey: walletState.publicKey,
        lamports,
        space: 8,
        programId: program.programId,
      })
    );

    await provider.sendAndConfirm(tx, [walletState]);
  });

  it("Enables transfers for a wallet", async () => {
    await program.methods
      .setTransferPermission(true)
      .accounts({
        walletState: walletState.publicKey,
        owner: walletOwner.publicKey,
      })
      .signers([walletOwner.payer])
      .rpc();

    const walletStateAccount = await program.account.walletState.fetch(walletState.publicKey);
    assert.isTrue(walletStateAccount.isTransferEnabled, "Transfer should be enabled");
  });

  it("Allows token transfer when enabled", async () => {
    await program.methods
      .transferHook(Buffer.from([])) // Empty buffer for mock input
      .accounts({
        walletState: walletState.publicKey,
        mint: walletOwner.publicKey, // Replace with actual mint
        extraAccountMetas: walletOwner.publicKey, // Replace with actual PDA
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .rpc();
  });

  it("Disables transfers for a wallet", async () => {
    await program.methods
      .setTransferPermission(false)
      .accounts({
        walletState: walletState.publicKey,
        owner: walletOwner.publicKey,
      })
      .signers([walletOwner.payer])
      .rpc();

    const walletStateAccount = await program.account.walletState.fetch(walletState.publicKey);
    assert.isFalse(walletStateAccount.isTransferEnabled, "Transfer should be disabled");
  });

  it("Fails token transfer when disabled", async () => {
    try {
      await program.methods
        .transferHook(Buffer.from([])) // Empty buffer for mock input
        .accounts({
          walletState: walletState.publicKey,
          mint: walletOwner.publicKey, // Replace with actual mint
          extraAccountMetas: walletOwner.publicKey, // Replace with actual PDA
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        })
        .rpc();

      assert.fail("Transfer should have been rejected, but it succeeded");
    } catch (err) {
      assert.include(err.message, "Token transfer is not allowed", "Transfer should fail");
    }
  });
});