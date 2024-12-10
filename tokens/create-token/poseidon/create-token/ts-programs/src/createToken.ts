import { Mint, Pubkey, Result, Signer, TokenProgram, u8 } from '@solanaturbine/poseidon';

export default class CreateToken {
  static PROGRAM_ID = new Pubkey('2GEjNvm8P1npWqX2ctzYtEkPpuJ5VFaDGQAQjdi9WiWF');

  createTokenMint(payer: Signer, mint: Mint, decimals: u8): Result {
    // Initialize the mint account with specified decimals

    mint.derive(null, payer.key, decimals).init(payer);
  }
}
