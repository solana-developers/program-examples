import { AssociatedTokenAccount, Mint, Pubkey, Signer, TokenProgram, u64 } from '@solanaturbine/poseidon';

export default class SplMintProgram {
  static PROGRAM_ID = new Pubkey('7drtUeP5AWkcXTN9jLMA9zpDNyGT3FCgbX96yMvuxFrJ');

  create(maker: Signer, tokenAccount: AssociatedTokenAccount, mint: Mint) {
    mint.initIfNeeded();
    TokenProgram.initializeAccount(tokenAccount, mint, maker);
  }

  mint(mintAccount: Mint, to: AssociatedTokenAccount, auth: Signer, amount: u64) {
    to.initIfNeeded();
    TokenProgram.mintTo(mintAccount, to, auth, amount);
  }
}
//    use anchor_spl::token_2022::{initialize_account3, mint_to, InitializeAccount3, MintTo};
