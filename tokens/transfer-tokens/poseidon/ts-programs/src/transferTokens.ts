import {
  Account,
  AssociatedTokenAccount,
  Collection,
  CreateMetadataV3,
  Creator,
  DataV2,
  Metadata,
  MetadataProgram,
  Mint,
  None,
  Pubkey,
  Signer,
  String,
  SystemAccount,
  SystemProgram,
  TokenAccount,
  TokenProgram,
  UncheckedAccount,
  UseMethod,
  Uses,
  Vec,
  u8,
  u16,
  u64,
} from '@solanaturbine/poseidon';
import { none, some } from '@solanaturbine/poseidon/index';

export default class TransferTokensProgram {
  static PROGRAM_ID = new Pubkey('4h2WWD9id7t75bNDwwWRoWYh759MDePhPZFiFJat9E9S');

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
    //pda constraint for the metadata account
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

  mintToken(mint_authority: Signer, mint_account: Mint, recipient: SystemAccount, associated_token_account: AssociatedTokenAccount, amount: u64) {
    associated_token_account.derive(mint_account, recipient.key).initIfNeeded(mint_authority);
    TokenProgram.mintTo(mint_account, associated_token_account, mint_authority, amount);
  }

  transferTokens(
    sender: Signer,
    mint_account: Mint,
    recipient: SystemAccount,
    sender_token_account: AssociatedTokenAccount,
    recipient_token_account: AssociatedTokenAccount,
    amount: u64,
  ) {
    recipient_token_account.derive(mint_account, recipient.key).initIfNeeded(sender);
    sender_token_account.derive(mint_account, recipient.key).initIfNeeded(sender);
    TokenProgram.transfer(sender_token_account, recipient_token_account, sender, amount);
  }
}
