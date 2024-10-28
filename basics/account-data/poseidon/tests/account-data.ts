import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AccountData } from "../target/types/account_data";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { assert } from "chai";

describe("account-data", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AccountData as Program<AccountData>;

  // Generate new user keypairs for testing
  const user = Keypair.generate();

  // Variables for storing PDA and bump
  let addressInfoPDA: PublicKey;
  let addressInfoBump: number;

  before(async () => {
    const latestBlockHash = await provider.connection.getLatestBlockhash();

    // Airdrop 1 SOL to the user that will be used for testing
    const airdropUser = await provider.connection.requestAirdrop(
      user.publicKey,
      1 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropUser,
    });

    // Derive PDA for the address info
    [addressInfoPDA, addressInfoBump] = await PublicKey.findProgramAddressSync(
      [user.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Creates an address info account", async () => {
    // Defined the address info data
    const addressInfo = {
      name: "John Doe",
      houseNum: 120,
      street: 9,
      cityZipCode: 12345,
    };

    // Invoke the Create Address Info instruction from the program
    // Parameters: houseNumber, streetNumber, cityZipCode, name
    await program.methods
      .createAddressInfo(
        addressInfo.houseNum,
        addressInfo.street,
        addressInfo.cityZipCode,
        addressInfo.name
      )
      .accountsPartial({
        payer: user.publicKey,
        addressInfo: addressInfoPDA,
      })
      .signers([user])
      .rpc();

    // Fetch the account information
    const account = await program.account.addressInfoState.fetch(
      addressInfoPDA
    );

    // Assertions to verify account data.
    assert.strictEqual(account.name, addressInfo.name, "Name does not match");
    assert.strictEqual(
      account.houseNumber,
      addressInfo.houseNum,
      "House Number does not match"
    );
    assert.strictEqual(
      account.streetNumber,
      addressInfo.street,
      "Street Number does not match"
    );
    assert.strictEqual(
      account.cityZipCode,
      addressInfo.cityZipCode,
      "City Zip Code does not match"
    );
  });
});
