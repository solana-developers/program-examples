import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { assert } from 'chai';
import { CloseAccount } from '../target/types/close_account';

describe('close-account', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CloseAccount as Program<CloseAccount>;
  const user = provider.wallet;

  const accountState = anchor.web3.PublicKey.findProgramAddressSync([anchor.utils.bytes.utf8.encode('account')], program.programId)[0];
  const someData = Math.floor(Math.random() * 100);

  it('Can initalize an account', async () => {
    await program.methods
      .initalize(someData)
      .accounts({
        user: user.publicKey,
      })
      .rpc();
    const acc = await program.account.accountState.fetchNullable(accountState);
    assert.notEqual(acc, null);
  });
  it('Can close an account', async () => {
    await program.methods
      .close()
      .accounts({
        user: user.publicKey,
      })
      .rpc();
    const acc = await program.account.accountState.fetchNullable(accountState);
    assert.equal(acc, null);
  });
});
