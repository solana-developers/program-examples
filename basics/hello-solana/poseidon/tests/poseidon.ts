import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { assert } from 'chai';
import { Poseidon } from '../target/types/poseidon';

describe('poseidon', () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Poseidon as Program<Poseidon>;

  it('Say hello!', async () => {
    const tx = await program.methods.hello().rpc();
    console.log('Transaction signature:', tx);

    const txInfo = await program.provider.connection.getTransaction(tx, {
      commitment: 'confirmed',
    });
    assert(txInfo !== null, 'Transaction should be confirmed');
  });

  it('Should emit correct logs', async () => {
    const tx = await program.methods.hello().rpc();
    const txInfo = await program.provider.connection.getTransaction(tx, {
      commitment: 'confirmed',
    });

    const logs = txInfo?.meta?.logMessages || [];
    assert(logs.some((log) => log.includes('Hello, Solana!')));
    assert(logs.some((log) => log.includes('Program ID:')));
    assert(logs.some((log) => log.includes('Timestamp:')));
  });

  it('Should handle multiple transactions', async () => {
    for (let i = 0; i < 3; i++) {
      const tx = await program.methods.hello().rpc();
      const txInfo = await program.provider.connection.getTransaction(tx, {
        commitment: 'confirmed',
      });
      assert(txInfo !== null, `Transaction ${i + 1} should be confirmed`);
    }
  });
});
