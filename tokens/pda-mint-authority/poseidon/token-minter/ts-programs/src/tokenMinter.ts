import { AssociatedTokenAccount, Mint, Pubkey, Result, Signer, SystemAccount, SystemProgram, TokenProgram, u8, u64 } from '@solanaturbine/poseidon';

export default class TokenMinter {
  static PROGRAM_ID = new Pubkey('5jVxRAH6W8C8SNdX3HUnabC1r3F9MxnNHfKTBe2DRXkT');

  createToken(mintAuthority: Signer, mintAccount: Mint, decimals: u8, freezeAuthority?: Pubkey): Result {
    mintAccount.derive(['mint']);
    // Initialize the mint account with specified decimals
    TokenProgram.initializeMint(
      mintAccount,
      mintAuthority, // authority
      decimals,
      freezeAuthority, // freeze authority
    );
  }

  mintToken(mintAuthority: Signer, mintAccount: Mint, recipient: SystemAccount, associatedTokenAccount: AssociatedTokenAccount, amount: u64): Result {
    associatedTokenAccount.derive(mintAccount, recipient.key).initIfNeeded(mintAuthority);
    mintAccount.derive(['mint']);
    TokenProgram.mintTo(mintAccount, associatedTokenAccount, mintAccount, amount, ['mint', mintAccount.getBump()]);
  }
}
