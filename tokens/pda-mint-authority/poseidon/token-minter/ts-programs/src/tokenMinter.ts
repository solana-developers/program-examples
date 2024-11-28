import { AssociatedTokenAccount, Mint, Pubkey, Result, Signer, SystemAccount, SystemProgram, TokenProgram, u8, u64 } from '@solanaturbine/poseidon';

export default class TokenMinter {
  static PROGRAM_ID = new Pubkey('2Ry3iUWABuQv8PTjgPwaM1CFHB8D8CtuX6EVzYXQ3PvE');

  createToken(payer: Signer, mintAccount: Mint, decimals: u8): Result {
    mintAccount.derive(['mint']);

    TokenProgram.initializeMint(
      mintAccount, // mint
      mintAccount, // mintAuthority
      decimals, // decimals
      mintAccount.key, // freezeAuthority
    );
  }

  mintToken(payer: Signer, mintAccount: Mint, associatedTokenAccount: AssociatedTokenAccount, amount: u64): Result {
    associatedTokenAccount.derive(mintAccount, payer.key).initIfNeeded(payer);
    mintAccount.derive(['mint']);
    TokenProgram.mintTo(mintAccount, associatedTokenAccount, mintAccount, amount, ['mint', mintAccount.getBump()]);
  }
}
