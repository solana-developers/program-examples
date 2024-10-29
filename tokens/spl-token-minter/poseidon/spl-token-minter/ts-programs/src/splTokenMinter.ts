import { AssociatedTokenAccount, Mint, Pubkey, Result, Signer, SystemAccount, SystemProgram, TokenProgram, u8, u64 } from '@solanaturbine/poseidon';

export default class SplTokenMinter {
  static PROGRAM_ID = new Pubkey('DmrXSUGWYaqtWg8sbi9JQN48yVZ1y2m7HvWXbND52Mcw');

  createTokenMint(mintAuthority: Signer, mintAccount: Mint, decimals: u8, freezeAuthority?: Pubkey): Result {
    // Initialize the mint account with specified decimals
    TokenProgram.initializeMint(
      mintAccount,
      decimals,
      mintAuthority, // authority
      freezeAuthority, // freeze authority
    );
  }

  mint(mintAuthority: Signer, mintAccount: Mint, recipient: SystemAccount, associatedTokenAccount: AssociatedTokenAccount, amount: u64): Result {
    associatedTokenAccount.derive(mintAccount, recipient.key).initIfNeeded();
    TokenProgram.mintTo(mintAccount, associatedTokenAccount, mintAuthority, amount);
  }
}
