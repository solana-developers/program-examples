import { Mint, Pubkey, Result, Signer, TokenProgram, u8 } from '@solanaturbine/poseidon';

export default class CreateToken {
  static PROGRAM_ID = new Pubkey('FThBfjqE8JBZYX8SdiJtDZwwGuVaQVMFfcR9JEsxS2A');

  createTokenMint(payer: Signer, mint: Mint, decimals: u8, freezeAuthority?: Pubkey): Result {
    // Initialize the mint account with specified decimals

    TokenProgram.initializeMint(
      mint,
      decimals,
      payer, // authority
      freezeAuthority, // freeze authority
    );
  }
}
