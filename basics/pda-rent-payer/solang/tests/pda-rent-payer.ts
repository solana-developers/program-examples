import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PdaRentPayer } from "../target/types/pda_rent_payer";
import { PublicKey } from "@solana/web3.js";

describe("pda-rent-payer", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet;
  const connection = provider.connection;

  const program = anchor.workspace.PdaRentPayer as Program<PdaRentPayer>;

  // Amount of additional lamports to fund the dataAccount with.
  const fundLamports = 1 * anchor.web3.LAMPORTS_PER_SOL;

  // Derive the PDA that will be used to initialize the dataAccount.
  const [dataAccountPDA, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("rent_vault")],
    program.programId
  );

  it("Initialize the Rent Vault", async () => {
    // Add your test here.
    const tx = await program.methods
      .new([bump], new anchor.BN(fundLamports))
      .accounts({ dataAccount: dataAccountPDA })
      .rpc();
    console.log("Your transaction signature", tx);

    const accountInfo = await connection.getAccountInfo(dataAccountPDA);
    console.log("AccountInfo Lamports:", accountInfo.lamports);
  });

  it("Create a new account using the Rent Vault", async () => {
    const newAccount = anchor.web3.Keypair.generate();
    const space = 100; // number of bytes

    // Get the minimum balance for the account to be rent exempt.
    const lamports = await connection.getMinimumBalanceForRentExemption(space);

    // Invoke the createNewAccount instruction on our program
    const tx = await program.methods
      .createNewAccount(new anchor.BN(lamports))
      .accounts({ dataAccount: dataAccountPDA })
      .remainingAccounts([
        {
          pubkey: newAccount.publicKey, // account to create by directly transferring lamports
          isWritable: true,
          isSigner: false,
        },
      ])
      .rpc();
    console.log("Your transaction signature", tx);

    const accountInfo = await connection.getAccountInfo(newAccount.publicKey);
    console.log("AccountInfo Lamports:", accountInfo.lamports);
  });
});
