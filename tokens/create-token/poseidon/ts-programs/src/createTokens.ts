import { CreateMetadataV3, DataV2, Metadata, MetadataProgram, Mint, None, Pubkey, Signer, String, u8, u16, u64 } from '@solanaturbine/poseidon';
import { none, some } from '@solanaturbine/poseidon/index';

export default class CreateToken {
  static PROGRAM_ID = new Pubkey('6xsCBfTAhmH8JQ6cXN69cvWaiDUG723k76MDMjt9fRah');

  createTokenMint(
    key: Pubkey,
    payer: Signer,
    mint_account: Mint,
    metadata_account: Metadata,
    token_decimals: u8,
    token_name: String<20>,
    token_symbol: String<15>,
    token_uri: String<100>,
    seller_fee_basis_points: u16,
  ) {
    mint_account.derive(token_decimals, payer.key, payer.key).init(payer);
    metadata_account.derive(['metadata', mint_account.key]);
    MetadataProgram.createMetadataAccountsV3(
      new CreateMetadataV3(
        payer,
        payer,
        payer,
        mint_account, // mint account
      ),
      new DataV2(token_name, token_symbol, token_uri, seller_fee_basis_points, none(), none(), none()),
      true,
      false,
      none(),
    );
  }
}
