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

export default class EscrowProgram {
  static PROGRAM_ID = new Pubkey("11111111111111111111111111111111");

  make(
    maker: Signer,
    escrow: EscrowState,
    makerAta: AssociatedTokenAccount,
    makerMint: Mint,
    takerMint: Mint,
    auth: UncheckedAccount,
    vault: TokenAccount,
    depositAmount: u64,
    offerAmount: u64,
    seed: u64
  ) {
    makerAta.derive(makerMint, maker.key);

    auth.derive(["auth"]);

    // Here like we mentioned in counter we are deriving a PDA for TokenAccount
    // <TokenAccount>.derive([...], <mint of the token acc>, <authority of the token acc>)
    vault.derive(["vault", escrow.key], makerMint, auth.key).init();

    // here we can see that we are deriving using seed(u64), so we would do change it to bytes by <arg>.toBytes() which makes it consumable for derive
    escrow.derive(["escrow", maker.key, seed.toBytes()]).init();

    escrow.authBump = auth.getBump();
    escrow.vaultBump = vault.getBump();
    escrow.escrowBump = escrow.getBump();

    escrow.maker = maker.key;
    escrow.amount = offerAmount;
    escrow.seed = seed;
    escrow.makerMint = makerMint.key;
    escrow.takerMint = takerMint.key;

    TokenProgram.transfer(
      makerAta, // from
      vault, // to
      maker, // authority
      depositAmount // amount to transfered
    );
  }

  refund(
    maker: Signer,
    makerAta: AssociatedTokenAccount,
    makerMint: Mint,
    auth: UncheckedAccount,
    vault: TokenAccount,
    escrow: EscrowState
  ) {
    makerAta.derive(makerMint, maker.key);
    escrow
      .derive(["escrow", maker.key, escrow.seed.toBytes()])
      .has([maker])
      .close(maker);

    auth.derive(["auth"]);

    vault.derive(["vault", escrow.key], makerMint, auth.key);

    // similar to system program transfer, we are using seeds as the last arguement as we are tranfering from a PDA
    TokenProgram.transfer(vault, makerAta, auth, escrow.amount, [
      "auth",
      escrow.authBump.toBytes(),
    ]);
  }

  take(
    taker: Signer,
    maker: SystemAccount,
    makerAta: AssociatedTokenAccount,
    takerAta: AssociatedTokenAccount,
    takerReceiveAta: AssociatedTokenAccount,
    makerMint: Mint,
    takerMint: Mint,
    auth: UncheckedAccount,
    vault: TokenAccount,
    escrow: EscrowState
  ) {
    // for AssociatedTokenAccount(takerAta) since its associated with a pubkey there is no need to pass the seeds list. we can just pass the mint and authority
    // <AssociatedTokenAccount>.derive(<mint of the token acc>, <authority of the token acc>)
    takerAta.derive(makerMint, taker.key).initIfNeeded(); // if you're not sure that the Ata will exist, just chain initIfNeeded method instead of init

    takerReceiveAta.derive(makerMint, taker.key).initIfNeeded();

    makerAta.derive(makerMint, maker.key);

    escrow
      .derive(["escrow", maker.key, escrow.seed.toBytes()])
      .has([maker, makerMint, takerMint]) // has method makes sure that all the pubkeys in the list which is the Custom_Acc(escrow) holds is same as Acc's pubkey in the function(in this case `take`) arguements
      .close(maker);

    auth.derive(["auth"]);

    vault.derive(["vault", escrow.key], makerMint, auth.key);

    TokenProgram.transfer(takerAta, makerAta, taker, escrow.amount);

    let seeds: Seeds = ["auth", escrow.authBump.toBytes()];

    TokenProgram.transfer(vault, takerReceiveAta, auth, escrow.amount, seeds);
  }
}

export interface EscrowState extends Account {
  maker: Pubkey;
  makerMint: Pubkey;
  takerMint: Pubkey;
  amount: u64;
  seed: u64;
  authBump: u8;
  escrowBump: u8;
  vaultBump: u8;
}
