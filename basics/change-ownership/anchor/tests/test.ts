import * as anchor from "@project-serum/anchor";
import { CreateSystemAccount } from "../target/types/create_system_account";


describe("Create a system account", () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.CreateSystemAccount as anchor.Program<CreateSystemAccount>;

  it("Create the account", async () => {

    const newKeypair = anchor.web3.Keypair.generate();
    
    await program.methods.createSystemAccount()
      .accounts({
        payer: wallet.publicKey,
        newAccount: newKeypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([wallet.payer, newKeypair])
      .rpc();

  });

  it("Change ownership for the account", async () => {

    const newKeypair = anchor.web3.Keypair.generate();

    let ix = anchor.web3.SystemProgram.assign({
        accountPubkey: newKeypair.publicKey,
        programId: program.programId,
    })

    await anchor.web3.sendAndConfirmTransaction(
        provider.connection, 
        new anchor.web3.Transaction().add(ix),
        [wallet.payer, newKeypair]
    );
});

it("Change it again using the System Program", async () => {

    const newKeypair = anchor.web3.Keypair.generate();

    let ix = anchor.web3.SystemProgram.assign({
        accountPubkey: newKeypair.publicKey,
        programId: anchor.web3.SystemProgram.programId,
    })

    await anchor.web3.sendAndConfirmTransaction(
        provider.connection, 
        new anchor.web3.Transaction().add(ix),
        [wallet.payer, newKeypair]
    );
});
});
