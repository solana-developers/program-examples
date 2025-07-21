import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { AblToken } from '../target/types/abl_token'

describe('abl-token', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.ABLToken as Program<AblToken>

  it('should run the program', async () => {
    // Add your test here.
  })
})
