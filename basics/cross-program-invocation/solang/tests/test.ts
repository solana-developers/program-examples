import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Lever } from "../target/types/lever";
import { Hand } from "../target/types/hand";

describe("cross-program-invocation", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Generate a new keypair for the data accounts for each program
  const dataAccountLever = anchor.web3.Keypair.generate();
  const dataAccountHand = anchor.web3.Keypair.generate();
  const wallet = provider.wallet;

  // The lever program and hand program
  const leverProgram = anchor.workspace.Lever as Program<Lever>;
  const handProgram = anchor.workspace.Hand as Program<Hand>;

  it("Initialize the lever!", async () => {
    // Initialize data account for the lever program
    const tx = await leverProgram.methods
      .new()
      .accounts({ dataAccount: dataAccountLever.publicKey })
      .signers([dataAccountLever])
      .rpc();
    console.log("Your transaction signature", tx);

    // Fetch the state of the data account
    const val = await leverProgram.methods
      .get()
      .accounts({ dataAccount: dataAccountLever.publicKey })
      .view();

    console.log("State:", val);
  });

  it("Pull the lever!", async () => {
    // Initialize data account for the hand program
    // This is required by Solang, but the account is not used
    const tx = await handProgram.methods
      .new()
      .accounts({ dataAccount: dataAccountHand.publicKey })
      .signers([dataAccountHand])
      .rpc();
    console.log("Your transaction signature", tx);

    // Call the pullLever instruction on the hand program, which invokes the lever program via CPI
    const tx2 = await handProgram.methods
      .pullLever(dataAccountLever.publicKey, "Chris")
      .accounts({ dataAccount: dataAccountHand.publicKey })
      .remainingAccounts([
        {
          pubkey: dataAccountLever.publicKey, // The lever program's data account, which stores the state
          isWritable: true,
          isSigner: false,
        },
        {
          pubkey: leverProgram.programId, // The lever program's program ID
          isWritable: false,
          isSigner: false,
        },
      ])
      .rpc({ skipPreflight: true });
    console.log("Your transaction signature", tx2);

    // Fetch the state of the data account
    const val = await leverProgram.methods
      .get()
      .accounts({ dataAccount: dataAccountLever.publicKey })
      .view();

    console.log("State:", val);
  });

  it("Pull it again!", async () => {
    // Call the pullLever instruction on the hand program, which invokes the lever program via CPI
    const tx = await handProgram.methods
      .pullLever(dataAccountLever.publicKey, "Ashley")
      .accounts({ dataAccount: dataAccountHand.publicKey })
      .remainingAccounts([
        {
          pubkey: dataAccountLever.publicKey, // The lever program's data account, which stores the state
          isWritable: true,
          isSigner: false,
        },
        {
          pubkey: leverProgram.programId, // The lever program's program ID
          isWritable: false,
          isSigner: false,
        },
      ])
      .rpc({ skipPreflight: true });

    console.log("Your transaction signature", tx);

    // Fetch the state of the data account
    const val = await leverProgram.methods
      .get()
      .accounts({ dataAccount: dataAccountLever.publicKey })
      .view();

    console.log("State:", val);
  });
});
