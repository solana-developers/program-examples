import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";
import type { CreateSystemAccountProgram } from "../target/types/create_system_account_program";
import { BankrunProvider } from "anchor-bankrun";
import { startAnchor } from "solana-bankrun";

const IDL = require("../target/idl/create_system_account.json");
const PROGRAM_ID = new PublicKey(IDL.address);


describe("Create a system account", async () => {
  const context = await startAnchor(
    "",
    [{ name: "create_system_account", programId: PROGRAM_ID }],
    []
  );
  const provider = new BankrunProvider(context);

  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;
  const program = anchor.workspace
    .CreateSystemAccountProgram as anchor.Program<CreateSystemAccountProgram>;

  it("Create the account", async () => {
    // Generate the public key from the seed and the programId
    const [accountState, _] = anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("account")],
      program.programId
    );

    await program.methods
      .createSystemAccount()
      .accounts({
        owner: wallet.publicKey,
      })
      .rpc();

    // Minimum balance for rent exemption for new account
    const lamports = await connection.getMinimumBalanceForRentExemption(0);

    // Check that the account was created
    const accountInfo = await connection.getAccountInfo(accountState);
    assert.isNotNull(accountInfo, "Account should be created");
    assert(
      accountInfo.lamports >= lamports,
      "Account must have the minimum amount of lamports required for rent"
    );
  });
});
