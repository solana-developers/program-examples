import * as anchor from '@coral-xyz/anchor';
import { HelloWorldProgram } from '../target/types/hello_world_program';

describe('hello-solana', () => {
  // Configure the Anchor provider & load the program IDL
  // The IDL gives you a typescript module
  //
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.HelloWorldProgram as anchor.Program<HelloWorldProgram>;

  it('Say hello!', async () => {
    // Just run Anchor's IDL method to build a transaction!
    //
    await program.methods.helloSolana().accounts({}).rpc();
  });
});
