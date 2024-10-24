import {
  Account,
  Pubkey,
  type Result,
  i64,
  u8,
  Signer,
  u64,
  Mint,
  TokenAccount,
  AssociatedTokenAccount,
  SystemAccount,
  TokenProgram,
  UncheckedAccount,
  Seeds,
} from "@solanaturbine/poseidon";

export default class EscrowProgram {
  static PROGRAM_ID = new Pubkey(
    "8b7Pshe6L28ee9oCTjGHCYTugFCQjKSBGfbaXZt9f3EF"
  );

  make_offer(
    maker: Signer,
    token_mint_a: Mint,
    token_mint_b: Mint,
    maker_token_account_a: AssociatedTokenAccount,
    vault: TokenAccount,
    auth: UncheckedAccount,
    escrow: Escrow,
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
    id: u64
  ): Result {
    //Derive PDAs for constraints
    maker_token_account_a.derive(token_mint_a, maker.key);

    auth.derive(["auth"]);
    vault.derive(["vault", escrow.key], token_mint_a, auth.key).init()
    escrow
      .derive(["escrow", maker.key, id.toBytes()]).init()

    //Set state of the account
    escrow.authBump = auth.getBump();
    escrow.vaultBump = vault.getBump();
    escrow.escrowBump = escrow.getBump();

    escrow.maker = maker.key;
    escrow.token_mint_a = token_mint_a.key;
    escrow.token_mint_b = token_mint_b.key;

    escrow.token_b_wanted_amount = token_b_wanted_amount;
    escrow.id = id;

    //Transfer offered tokens to vault
    TokenProgram.transfer(
      maker_token_account_a, //from
      vault, //to
      maker, // authority
      token_a_offered_amount // amount to be transferred
    );
  }

  take_offer(
    taker: Signer,
    maker: SystemAccount,
    maker_token_account_a:AssociatedTokenAccount,
    taker_token_account_a: AssociatedTokenAccount,
    taker_token_account_b: AssociatedTokenAccount,
    token_mint_a: Mint,
    token_mint_b: Mint,
    auth: UncheckedAccount,
    vault: TokenAccount,
    escrow: Escrow,
  ): Result {
    taker_token_account_a.derive(token_mint_a, taker.key).initIfNeeded(); // if you're not sure that the Ata will exist, just chain initIfNeeded method instead of init

    taker_token_account_b.derive(token_mint_a, taker.key).initIfNeeded();

    maker_token_account_a.derive(token_mint_a, maker.key);

    //Close account and send lamports to maker
    escrow
      .derive(["escrow", maker.key, escrow.id.toBytes()])
      .has([maker, token_mint_a, token_mint_b]) // the has method will make sure that all the pubkeys in the list which is the Custom_Acc(escrow) holds is same as Acc's pubkey in the function(in this case `take`) arguements
      .close(maker);

    //Derive PDAs for constraints
    auth.derive(["auth"]);

    vault.derive(["vault", escrow.key], token_mint_a, auth.key)

    // Send wanted tokens to maker
    TokenProgram.transfer(
      taker_token_account_a,
      maker_token_account_a,
      taker,
      escrow.token_b_wanted_amount
    );

    let seeds:Seeds = ["auth", escrow.authBump.toBytes()]

    TokenProgram.transfer(
        vault, 
        taker_token_account_b,
        auth,
        escrow.token_b_wanted_amount,
        seeds
    );
  }
}

export interface Escrow extends Account {
  maker: Pubkey;
  token_mint_a: Pubkey;
  token_mint_b: Pubkey;
  token_b_wanted_amount: u64;
  escrowBump: u8;
  id: u64;
  authBump: u8;
  vaultBump: u8;
}
