import * as anchor from "@project-serum/anchor";
import { RentExample, IDL } from "../target/types/rent_example";


describe("Create a system account", () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.RentExample as anchor.Program<RentExample>;

  it("Create the account", async () => {

    const newKeypair = anchor.web3.Keypair.generate();

    const addressData: anchor.IdlTypes<RentExample>["AddressData"] = {
      name: "Marcus",
      address: "123 Main St. San Francisco, CA"
    };

    const addressDataBuffer = new anchor.BorshCoder(IDL).types.encode("AddressData", addressData);
    console.log(`Address data buffer length: ${addressDataBuffer.length}`);
    
    await program.methods.createSystemAccount(addressData)
    .accounts({
      payer: wallet.publicKey,
      newAccount: newKeypair.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .signers([wallet.payer, newKeypair])
    .rpc();

  });
});
