import * as anchor from '@coral-xyz/anchor';
import type { HelloSolana } from '../target/types/hello_solana';
import { BN, Program, web3 } from '@project-serum/anchor'
import { PublicKey } from '@solana/web3.js';
import * as assert from 'assert';


describe('hello_solana', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.HelloSolana as anchor.Program<HelloSolana>;
  const payer = provider.wallet as anchor.Wallet;
  const owner: anchor.web3.PublicKey = payer.publicKey

  console.log("Provider RPC URL:", provider.connection.rpcEndpoint);
  console.log("Program ID:", program.programId.toBase58());

  // Derive PDA, which is the account hosting the program data
  const [hello_solana, _] = PublicKey.findProgramAddressSync(
    [Buffer.from('Message'), owner.toBuffer()],
    program.programId
  );

  console.log("Derived PDA:", hello_solana.toBase58());

  it('Init HelloSolana', async() => {

    // Check if PDA already exists
    const accountInfo = await provider.connection.getAccountInfo(hello_solana);
    console.log("Initial PDA Account Info:", accountInfo);

    if (accountInfo) {
      console.log("Account already exists, skipping initialization.");
    } else {
      console.log("Account does not exist, proceeding with initialization.");
      

      await program.methods.initialize().accounts({ owner, hello_solana, systemProgram: anchor.web3.SystemProgram.programId, }).rpc();
      
      // Verify PDA after initialization
      const newAccountInfo = await provider.connection.getAccountInfo(hello_solana);
      if (newAccountInfo) {
        console.log("PDA created successfully.");
      } else {
        console.log("PDA still not found; check initialization logic.");
      }
      // Get the message value on chain
      const msg = await program.account.message.fetch(hello_solana)
      assert.ok (msg.value === "")

    }
  });

  it('Say hello!', async () => {
    await program.methods
      .hello().accounts({owner: owner, message: hello_solana}).rpc();

    // Get the message value on chain
    const msg = await program.account.message.fetch(hello_solana)

    console.log("Message retrieved is:'" + msg.value + "'")
    assert.ok(msg.value == 'Hello GM!')
  });

});
