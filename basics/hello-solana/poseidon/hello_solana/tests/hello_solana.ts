import { Idl, Program } from '@coral-xyz/anchor';
import { BankrunProvider } from 'anchor-bankrun';
import { test } from 'mocha';
import { startAnchor } from 'solana-bankrun';
import HelloSolanaIDL from '../target/idl/hello_solana.json';
import { HelloSolana } from '../target/types/hello_solana';

test('hello_solana', async () => {
  const context = await startAnchor('../hello_solana', [], []);
  const provider = new BankrunProvider(context);
  const helloSolanaProgram = new Program(HelloSolanaIDL as Idl, provider) as Program<HelloSolana>;

  await helloSolanaProgram.methods.hello().rpc();
});
