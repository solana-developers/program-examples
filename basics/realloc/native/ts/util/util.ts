import { Keypair } from '@solana/web3.js';

export function createKeypairFromFile(path: string): Keypair {
  return Keypair.fromSecretKey(Buffer.from(JSON.parse(require('node:fs').readFileSync(path, 'utf-8'))));
}
