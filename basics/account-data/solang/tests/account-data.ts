import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AccountData } from "../target/types/account_data";

describe("account-data", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Generate a new random keypair for the data account.
  const dataAccount = anchor.web3.Keypair.generate();
  const wallet = provider.wallet;
  const program = anchor.workspace.AccountData as Program<AccountData>;

  // Create the new account
  // Using 10240 bytes of space, because its unclear how to correctly calculate the minimum space needed for the account
  // Space calculation is different from regular Native/Anchor Solana programs
  it("Is initialized!", async () => {
    const tx = await program.methods
      .new(
        10240, // space (10240 bytes is the maximum space allowed when allocating space through a program)
        "Joe C", // name
        136, // house number
        "Mile High Dr.", // street
        "Solana Beach" // city
      )
      .accounts({ dataAccount: dataAccount.publicKey })
      .signers([dataAccount])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  // Get the account data
  it("Get AddressInfo Data", async () => {
    const val = await program.methods
      .get()
      .accounts({ dataAccount: dataAccount.publicKey })
      .view();
    console.log("State:", val);
  });

  // Get the account data size
  // Testing how much space is used to store the account data
  // However, minimum space required is greater than this
  it("Get AddressInfo Size", async () => {
    const size = await program.methods
      .getAddressInfoSize()
      .accounts({ dataAccount: dataAccount.publicKey })
      .view();
    console.log("Size:", size.toNumber());
  });
});
