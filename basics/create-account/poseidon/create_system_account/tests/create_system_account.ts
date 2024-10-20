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
    assert(accountInfo.lamports >= lamports, "Account must have the minimum amount of lamports required for rent");
  });
});
