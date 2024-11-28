import * as anchor from "@coral-xyz/anchor";
import { assert } from "chai";
import type { RentProgram } from "../target/types/rent_program";

describe("Check that the required amount of rent has been allocated", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;
  const program = anchor.workspace.RentProgram as anchor.Program<RentProgram>;

  it("Create an account that holds the amount required for rent exemption", async () => {
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

    // Specify the size of the account and retrieve the required lamports for rent
    const lamports = await connection.getMinimumBalanceForRentExemption(44);

    // Check that the account have minimum amount of lamports required for rent
    const accountInfo = await connection.getAccountInfo(accountState);
    assert.isNotNull(accountInfo, "Account should be created");
    assert(
      accountInfo.lamports >= lamports,
      "Account must have the minimum amount of lamports required for rent"
    );
  });
});
