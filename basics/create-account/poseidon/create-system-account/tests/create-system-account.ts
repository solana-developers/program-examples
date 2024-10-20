import * as anchor from "@coral-xyz/anchor";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";
import type { CreateSystemAccountProgram } from "../target/types/create_system_account_program";

describe("Create a system account", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;
  const program = anchor.workspace
    .CreateSystemAccountProgram as anchor.Program<CreateSystemAccountProgram>;

  it("Creates the account with correct parameters", async () => {
    // Generate the public key from the seed and the programId
    const [accountState, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("account")],
      program.programId
    );

    await program.methods
      .createSystemAccount()
      .accounts({
        owner: wallet.publicKey,
        account: accountState,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    // Minimum balance for rent exemption for new account
    const lamports = await connection.getMinimumBalanceForRentExemption(41); // Adjust space to 41

    // Verify account creation
    const accountInfo = await connection.getAccountInfo(accountState);
    assert.isNotNull(accountInfo, "Account should be created");
    assert(
      accountInfo.lamports >= lamports,
      "Account must have the minimum amount of lamports required for rent"
    );

    // Fetch the on-chain account state
    const fetchedAccount = await program.account.accountState.fetch(
      accountState
    );
    assert.strictEqual(
      fetchedAccount.owner.toString(),
      wallet.publicKey.toString(),
      "Owner should be correctly set"
    );
    assert.strictEqual(fetchedAccount.accountBump, bump, "Bump should match");
  });

  it("Fails to create the account with insufficient lamports", async () => {
    // Create a new wallet with 0 lamports (insufficient funds)
    const insufficientWallet = Keypair.generate();
    const insufficientProvider = new anchor.AnchorProvider(
      connection,
      new anchor.Wallet(insufficientWallet),
      anchor.AnchorProvider.defaultOptions()
    );
    anchor.setProvider(insufficientProvider);

    const [accountState, _] = anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("account")],
      program.programId
    );

    try {
      await program.methods
        .createSystemAccount()
        .accounts({
          owner: insufficientWallet.publicKey,
          account: accountState,
          systemProgram: SystemProgram.programId,
        })
        .signers([insufficientWallet])
        .rpc();
      assert.fail("Account creation should fail due to insufficient funds");
    } catch (err) {
      assert.include(
        err.message,
        "insufficient funds",
        "Expected error message"
      );
    }
  });

  it("Fails to create the account with wrong seeds", async () => {
    const [wrongAccountState, _] = anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("wrong_account")], // Using incorrect seed
      program.programId
    );

    try {
      await program.methods
        .createSystemAccount()
        .accounts({
          owner: wallet.publicKey,
          account: wrongAccountState,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      assert.fail("Account creation should fail due to incorrect seed");
    } catch (err) {
      assert.include(
        err.message,
        "failed to find account",
        "Expected seed mismatch error"
      );
    }
  });

  it("Validates that the system program is included", async () => {
    const [accountState, _] = anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("account")],
      program.programId
    );

    try {
      await program.methods
        .createSystemAccount()
        .accounts({
          owner: wallet.publicKey,
          account: accountState,
          // Intentionally omitting systemProgram to trigger an error
        })
        .rpc();
      assert.fail("Account creation should fail without system program");
    } catch (err) {
      assert.include(
        err.message,
        "ProgramNotProvided",
        "Expected missing system program error"
      );
    }
  });
});
