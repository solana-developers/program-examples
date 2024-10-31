import { AssociatedTokenAccount, Mint, Pubkey, Result, Signer, SystemAccount, SystemProgram, TokenProgram, u8, u64 } from '@solanaturbine/poseidon';

export default class SplTokenMinter {
  static PROGRAM_ID = new Pubkey('CSi4VcU9g99HKSodPV3MJvweoEAuaqWqgEC3jvdHieDG');

  createTokenMint(mintAuthority: Signer, mintAccount: Mint, decimals: u8, freezeAuthority?: Pubkey): Result {
    // Initialize the mint account with specified decimals
    TokenProgram.initializeMint(
      mintAccount,
      mintAuthority, // authority
      decimals,
      freezeAuthority, // freeze authority
    );
  }

  mint(mintAuthority: Signer, mintAccount: Mint, recipient: SystemAccount, associatedTokenAccount: AssociatedTokenAccount, amount: u64): Result {
    associatedTokenAccount.derive(mintAccount, recipient.key).initIfNeeded(mintAuthority);
    TokenProgram.mintTo(mintAccount, associatedTokenAccount, mintAuthority, amount);
  }
}
