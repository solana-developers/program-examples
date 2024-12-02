import {
  Account,
  AssociatedTokenAccount,
  Mint,
  String as PoseidonString,
  Pubkey,
  Seeds,
  Signer,
  SystemAccount,
  TokenAccount,
  TokenProgram,
  UncheckedAccount,
  u8,
  u64,
} from '@solanaturbine/poseidon';

export default class TokenMinter {
  static PROGRAM_ID = new Pubkey('AMXNdYTyDpcLLJ9CzVJQ1kw5gqE4JeZxjtUbH2MwntdD');

  //Creating token metadata is not supported in poseidon currently
  createToken(
    maker: Signer,
    makerMint: Mint,
    makerAssociatedTokenAccount: AssociatedTokenAccount,
    auth: UncheckedAccount,
    token_name: PoseidonString<10>,
    token_symbol: PoseidonString<10>,
    token_uri: PoseidonString<10>,
  ) {
    //create_metadata_accounts_v3 function not yet implemented in poseidon
  }
  mint(payer: Signer, makerMint: Mint, makerAta: AssociatedTokenAccount) {
    makerMint.derive(['mint']);
    makerAta.derive(makerMint, payer.key).initIfNeeded(payer);
    TokenProgram.initializeMint(makerMint, new u8(8), payer);
  }
}

export interface CreateToken extends Account {}

export interface MintToken extends Account {
  maker: Pubkey;
  makerMintAccount: Pubkey;
  makerMintBump: u8;
  seed: u64;
  authBump: u8;
  amount: u64;
}
