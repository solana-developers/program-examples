import { AssociatedTokenAccount, Mint, Pubkey, type Result, Signer, TokenProgram, u8, u64 } from '@solanaturbine/poseidon';

// Add "anchor-spl/idl-build" to idl-build list in Cargo.toml
// Change "CpiContext::new_with_signer" at mint function to "CpiContext::new" and remove the signer parameter

// use anchor_lang::prelude::*;
// use anchor_spl::{
//     token::{transfer as transfer_spl, Transfer as TransferSPL, mint_to, Mint, MintTo, Token, TokenAccount},
//     associated_token::AssociatedToken,
// };

// Add "mut" inside the #[account()]
// under "to_account" & "mint_account" under "MintContext"

export default class SplTokenMinter {
  static PROGRAM_ID = new Pubkey('HFKNWrbYAfKsrWJu88RtUVHgVBNz1uJ6u2tNx1YCmAMZ');

  createToken(mint: Mint, decimals: u8, payer: Signer, freezeAuthority: Pubkey): Result {
    mint.initIfNeeded();
    TokenProgram.initializeMint(mint, decimals, payer, freezeAuthority);
  }

  mint(mintAccount: Mint, toAccount: AssociatedTokenAccount, signer: Signer, amount: u64): Result {
    toAccount.initIfNeeded();
    TokenProgram.mintTo(mintAccount, toAccount, signer, amount);
  }
}
