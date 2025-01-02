import * as anchor from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { BankrunProvider } from "anchor-bankrun";
import { assert } from "chai";
import { startAnchor } from "solana-bankrun";
import { EscrowProgram } from "../target/types/escrow_program";

const IDL = require("../target/idl/escrow_program.json");
const PROGRAM_ID = new PublicKey(IDL.address);

describe("escrow_program (Bankrun)", async () => {
  const context = await startAnchor(
    "",
    [{ name: "escrow_program", programId: PROGRAM_ID }],
    []
  );
  const provider = new BankrunProvider(context);

  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<EscrowProgram>(IDL, provider);

  // Generate keypairs for Maker, Taker, and the escrow account
  const maker = Keypair.generate();
  const taker = Keypair.generate();
  const escrowKeypair = new Keypair();

  const mint = new Keypair(); // Mint account for SPL tokens
  let makerATA, takerATA, escrowVault, escrowState;

  it("Make Escrow", async () => {
    await program.methods
      .make(new anchor.BN(100), new anchor.BN(50), new anchor.BN(12345)) // deposit_amount, offer_amount, seed
      .accounts({
        escrow: escrowKeypair.publicKey,
        maker: maker.publicKey,
        makerMint: mint.publicKey,
        takerMint: mint.publicKey,
        makerAta: makerATA,
        vault: escrowVault,
        auth: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      })
      .signers([maker, escrowKeypair])
      .rpc();

    // Fetch and verify the state of the escrow
    escrowState = await program.account.escrowState.fetch(
      escrowKeypair.publicKey
    );
    assert.equal(escrowState.maker.toString(), maker.publicKey.toString());
    assert.equal(escrowState.amount.toNumber(), 50);
  });

  it("Refund Escrow", async () => {
    await program.methods
      .refund()
      .accounts({
        escrow: escrowKeypair.publicKey,
        maker: maker.publicKey,
        vault: escrowVault,
        auth: provider.wallet.publicKey,
        makerAta: makerATA,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([maker])
      .rpc();

    // Assert that escrow is closed or funds refunded
    escrowState = await program.account.escrowState
      .fetch(escrowKeypair.publicKey)
      .catch(() => null);
    assert.isNull(escrowState, "Escrow state should be closed after refund");
  });

  it("Take Escrow", async () => {
    await program.methods
      .take()
      .accounts({
        escrow: escrowKeypair.publicKey,
        taker: taker.publicKey,
        maker: maker.publicKey,
        makerAta: makerATA,
        takerAta: takerATA,
        vault: escrowVault,
        auth: provider.wallet.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([taker])
      .rpc();

    // Check token balances and transfers
    // Maker and Taker balances should reflect the correct amounts.
  });
});
