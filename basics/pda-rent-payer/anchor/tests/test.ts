import * as anchor from "@coral-xyz/anchor";
import { PdaRentPayer } from "../target/types/pda_rent_payer";

describe("PDA Rent-Payer", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.PdaRentPayer as anchor.Program<PdaRentPayer>;

  function deriveRentVaultPda() {
    const pda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("rent_vault")],
      program.programId
    );
    console.log(`PDA: ${pda[0].toBase58()}`);
    return pda;
  }

  it("Initialize the Rent Vault", async () => {
    const [rentVaultPda, _] = deriveRentVaultPda();
    await program.methods
      .initRentVault(new anchor.BN(1000000000))
      .accounts({
        rentVault: rentVaultPda,
        payer: wallet.publicKey,
      })
      .signers([wallet.payer])
      .rpc();
  });

  it("Create a new account using the Rent Vault", async () => {
    const newAccount = anchor.web3.Keypair.generate();
    const [rentVaultPda, _] = deriveRentVaultPda();
    await program.methods
      .createNewAccount()
      .accounts({
        newAccount: newAccount.publicKey,
        rentVault: rentVaultPda,
      })
      .signers([wallet.payer])
      .rpc();
  });
});
