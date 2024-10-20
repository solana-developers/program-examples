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

export default class PdaMintAuthorityrogram {
  static PROGRAM_ID = new Pubkey("11111111111111111111111111111111");

  create(
    maker: Signer,
    makerMint: Mint,
    makerAta: AssociatedTokenAccount,
    auth: UncheckedAccount,
    seed: u64
  ) {

  }
  mint(
    maker: Signer,
    makerMint: Mint,
    makerAta: AssociatedTokenAccount,
    auth: UncheckedAccount,
  ) {

  }
}


export interface PdaMintState extends Account {
  maker: Pubkey;
  makerMint: Pubkey;
  amount: u64;
  seed: u64;
  authBump: u8;
  escrowBump: u8;
}
