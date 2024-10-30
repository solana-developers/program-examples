import { Mint, Pubkey, Result, Signer, TokenProgram, u8 } from '@solanaturbine/poseidon';

export default class CreateToken {
  static PROGRAM_ID = new Pubkey('7ZpQnmMWwNbuSRnBpq2E4RTKMgN5tDNopF7BHvSJZfwU');

  createTokenMint(payer: Signer, mint: Mint, decimals: u8, freezeAuthority?: Pubkey): Result {
    // Initialize the mint account with specified decimals

    TokenProgram.initializeMint(
      mint,
      payer, // authority
      decimals,
      freezeAuthority, // freeze authority
    );
  }
}
