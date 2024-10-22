import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { HelloSolana } from '../target/types/hello_solana'

describe('hello-solana', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.HelloSolana as Program<HelloSolana>

  it('Say hello!', async () => {
    // Add your test here.
    await program.methods.hello().accounts({}).rpc()
  })
})
