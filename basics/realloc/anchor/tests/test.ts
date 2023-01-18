import * as anchor from "@project-serum/anchor";
import { AnchorProgramExample } from "../target/types/anchor_program_example";



describe("Realloc!", async () => {

    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const payer = provider.wallet as anchor.Wallet;
    const program = anchor.workspace.AnchorProgramExample as anchor.Program<AnchorProgramExample>;

    const testAccount = anchor.web3.Keypair.generate();

  it("Create the account with data", async () => {
      console.log(`${testAccount.publicKey}`);
      await program.methods.createAddressInfo(
        "Jacob",
        123,
        "Main St.",
        "Chicago",
      )
        .accounts({
            targetAccount: testAccount.publicKey,
            payer: payer.publicKey, 
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([payer.payer, testAccount])
        .rpc();
      await printAddressInfo(testAccount.publicKey);
  });
  
  it("Reallocate WITHOUT zero init", async () => {
      await program.methods.reallocateWithoutZeroInit(
        "Illinois",
        12345,
      )
        .accounts({
            targetAccount: testAccount.publicKey,
            payer: payer.publicKey, 
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([payer.payer])
        .rpc();
      await printEnhancedAddressInfo(testAccount.publicKey);
  });

  it("Reallocate WITH zero init", async () => {
      await program.methods.reallocateZeroInit(
        "Pete",
        "Engineer",
        "Metaplex",
        2,
      )
        .accounts({
            targetAccount: testAccount.publicKey,
            payer: payer.publicKey, 
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([payer.payer])
        .rpc();
      await printWorkInfo(testAccount.publicKey);
  });

  async function printAddressInfo(pubkey: anchor.web3.PublicKey): Promise<void> {
      await delay(2);
      const addressInfo = await program.account.addressInfo.fetch(pubkey);
      if (addressInfo) {
          console.log("Address info:");
          console.log(`   Name:       ${addressInfo.name}`);
          console.log(`   House Num:  ${addressInfo.house_number}`);
          console.log(`   Street:     ${addressInfo.street}`);
          console.log(`   City:       ${addressInfo.city}`);
      };
  }

  async function printEnhancedAddressInfo(pubkey: anchor.web3.PublicKey): Promise<void> {
      await delay(2);
      const enhancedAddressInfo = await program.account.enhancedAddressInfo.fetch(pubkey);
      if (enhancedAddressInfo) {
          console.log("Enhanced Address info:");
          console.log(`   Name:       ${enhancedAddressInfo.name}`);
          console.log(`   House Num:  ${enhancedAddressInfo.house_number}`);
          console.log(`   Street:     ${enhancedAddressInfo.street}`);
          console.log(`   City:       ${enhancedAddressInfo.city}`);
          console.log(`   State:      ${enhancedAddressInfo.city}`);
          console.log(`   Zip:        ${enhancedAddressInfo.city}`);
      };
  }

  async function printWorkInfo(pubkey: anchor.web3.PublicKey): Promise<void> {
      await delay(2);
      const workInfo = await program.account.workInfo.fetch(pubkey);
      if (workInfo) {
          console.log("Work info:");
          console.log(`   Name:       ${workInfo.name}`);
          console.log(`   Position:   ${workInfo.position}`);
          console.log(`   Company:    ${workInfo.company}`);
          console.log(`   Years:      ${workInfo.years_employed}`);
      };
  }

  function delay(s: number) {
      return new Promise( resolve => setTimeout(resolve, s * 1000) );
  }
});
