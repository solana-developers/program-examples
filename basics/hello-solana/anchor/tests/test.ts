import * as anchor from '@coral-xyz/anchor';
import { HelloSolana } from '../target/types/hello_solana';

describe('hello-solana', () => {
  // Configure the Anchor provider & load the program IDL
  // The IDL gives you a typescript module
  //
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.HelloSolana as anchor.Program<HelloSolana>;

  it('Say hello!', async () => {
    // Just run Anchor's IDL method to build a transaction!
    //
    await program.methods.hello().accounts({}).rpc();
  });
});
