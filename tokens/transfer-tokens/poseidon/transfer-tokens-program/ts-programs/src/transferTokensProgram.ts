import { AssociatedTokenAccount, Mint, Pubkey, Signer, TokenProgram, u64 } from '@solanaturbine/poseidon';

export default class TokenTransferProgram {
  static PROGRAM_ID = new Pubkey('BSHN8q3tEDsSiHBEHKKgxevQoSvXpKciZ7W3kcWSuEfC');

  transferTokens(
    owner: Signer,
    sourceAta: AssociatedTokenAccount,
    destination: Pubkey,
    destinationAta: AssociatedTokenAccount,
    mint: Mint,
    transferAmount: u64,
  ) {
    sourceAta.derive(mint, owner.key).initIfNeeded();
    destinationAta.derive(mint, destination).initIfNeeded();

    TokenProgram.transfer(sourceAta, destinationAta, owner, transferAmount);
  }
}
