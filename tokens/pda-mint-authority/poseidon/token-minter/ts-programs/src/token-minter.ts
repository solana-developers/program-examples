import {
  Account,
  AssociatedTokenAccount,
  Mint,
  Pubkey,
  Seeds,
  Signer,
  SystemAccount,
  TokenAccount,
  TokenProgram,
  UncheckedAccount,
  u64,
  u8,
} from "@solanaturbine/poseidon";

export default class TokenMinter {
  static PROGRAM_ID = new Pubkey(
    "EWEURHBPCLgFnxMV6yKmmj2xS9386Rcr2ixBah8Pyjjv"
  );

//Creating token metadata is not supported in poseidon currently so that will be done in the tests
//so for creating metadata and signing as pda, we will have to populate the anchor code manually 
//  createToken(
//     maker: Signer,
//     makerMint: Mint,
//     makerAssociatedTokenAccount: AssociatedTokenAccount,
//     auth: UncheckedAccount,
//     token_name: String,
//     token_symbol: String,
//     token_uri: String,
//  ) {
//  }
  mintToken(
    maker: Signer,
    makerMintAccount: Mint,
    makerAssociatedTokenAccount: AssociatedTokenAccount,
    auth: UncheckedAccount,
    amount: u64
  ) {

    TokenProgram.initializeMint(
        
    )
    makerMintAccount.derive(["mint"]);

    makerAssociatedTokenAccount
      .derive(makerMintAccount, auth.key)
      .has([maker, makerMintAccount])
      .initIfNeeded();
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
