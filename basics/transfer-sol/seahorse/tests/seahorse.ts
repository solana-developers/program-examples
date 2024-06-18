import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import type { Seahorse } from '../target/types/seahorse';

describe('seahorse', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Seahorse as Program<Seahorse>;

  const mockReceiverAccount = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('mock_account')], program.programId);

  it('Initialize the Mock account to send our SOL to', async () => {
    const tx = await program.methods
      .initMockAccount()
      .accounts({
        mockAccount: mockReceiverAccount[0],
        signer: program.provider.publicKey,
      })
      .rpc();
  });
  it('Send SOL To Mock account', async () => {
    const transferAmount = 1;
    // Convert to lamport.
    const lamports: number = anchor.web3.LAMPORTS_PER_SOL * transferAmount;
    const tx = await program.methods
      .transferSolWithCpi(new anchor.BN(lamports))
      .accounts({
        recipient: mockReceiverAccount[0],
        sender: program.provider.publicKey,
      })
      .rpc();
    console.log('Your transaction signature: ', tx);
  });
});
