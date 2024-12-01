import {
  Account,
  AssociatedTokenAccount,
  Constraint,
  Mint,
  PoseidonError,
  Pubkey,
  Seeds,
  Signer,
  TokenAccount,
  TokenProgram,
  i64,
  u8,
  u16,
  u64,
} from '@solanaturbine/poseidon';

//Poseidon cannot support custom instructions currently , so most of the amm logic has been commented out
export default class tokenSwap {
  static PROGRAM_ID = new Pubkey('3dDaJxmPcmQVfSx9rX4xHyP5rJvkwdKcNujcX2z9KB9h');

  create_amm(payer: Signer, amm: AMM, admin: Admin, id: u64, fee: u16) {
    amm
      .derive([id.toBytes()])
      //Custom constraints don't transpile to corresponding anchor constraints yet
      .constraints([new Constraint(fee < new u16(10000), new PoseidonError('invalid fee'))])
      .init();
    admin.derive(['admin']).init();
    amm.id = id;
    amm.admin = admin.key;
    amm.fee = fee;
  }

  create_pool(
    payer: Signer,
    amm: AMM,
    pool: Pool,
    pool_authority: PoolAuthority,
    pool_account_a: AssociatedTokenAccount,
    pool_account_b: AssociatedTokenAccount,
    mint_liquidity: Mint,
    mint_a: Mint,
    mint_b: Mint,
    id: u64,
  ) {
    amm.derive([id.toBytes()]).init();
    pool.derive([amm.key, mint_a.key, mint_b.key]).init();
    pool_authority.derive([amm.key, mint_a.key, mint_b.key, 'authority']);
    mint_liquidity.derive([amm.key, mint_a.key, mint_b.key, 'liquidity']).init();
    pool_account_a.derive(mint_a, pool_authority.key).init();
    pool_account_b.derive(mint_b, pool_authority.key).init();

    pool.amm = amm.key;
    pool.mint_a = mint_a.key;
    pool.mint_b = mint_b.key;
  }

  // The liquidity is a constant value here for testing purposes since theres no way to make custom logic
  deposit_liquidity(
    payer: Signer,
    depositor: Signer,
    pool: Pool,
    pool_authority: PoolAuthority,
    pool_account_a: AssociatedTokenAccount,
    pool_account_b: AssociatedTokenAccount,
    depositor_account_a: AssociatedTokenAccount,
    depositor_account_b: AssociatedTokenAccount,
    depositor_account_liquidity: AssociatedTokenAccount,
    amm: AMM,
    mint_liquidity: Mint,
    mint_a: Mint,
    mint_b: Mint,
    amount_a: u64,
    amount_b: u64,
  ) {
    pool.derive([amm.key, mint_a.key, mint_b.key]).has([mint_a, mint_b]);

    pool_authority.derive([amm.key, mint_a.key, mint_b.key, 'authority']);

    mint_liquidity.derive([amm.key, mint_a.key, mint_b.key, 'liquidity']).init();

    pool_account_a.derive(mint_a, pool_authority.key).init();
    pool_account_b.derive(mint_b, pool_authority.key).init();

    depositor_account_liquidity.derive(mint_liquidity, depositor.key).init();

    depositor_account_a.derive(mint_a, depositor.key).init();
    depositor_account_b.derive(mint_b, depositor.key).init();

    //   prevent depositing assets the depositor does not own
    //   let amount_a = new i64(amount_a); // Set from actual initial value for `amount_a`
    //   let amount_b = new i64(amount_b); // Set from actual initial value for `amount_b`
    //   const depositor_account_a_amount = new i64(depositor_account_a.amount);
    //   const depositor_account_b_amount = new i64(depositor_account_b.amount);

    //   // Limit `amount_a` and `amount_b` to the depositor account balances
    //   if (amount_a.gt(depositor_account_a_amount)) {
    //     amount_a = depositor_account_a_amount;
    //   }
    //   if (amount_b.gt(depositor_account_b_amount)) {
    //     amount_b = depositor_account_b_amount;
    //   }

    //   // Define pool account balances
    //   const pool_account_a_amount = new i64(pool_account_a.amount);
    //   const pool_account_b_amount = new i64(pool_account_b.amount);

    //   // Check if pool creation is happening (no liquidity yet)
    //   const pool_creation =
    //     pool_account_a_amount.eq(new i64(0)) &&
    //     pool_account_b_amount.eq(new i64(0));

    //   // Calculate `amount_a` and `amount_b` based on existing liquidity
    //   if (pool_creation) {
    //     // If creating a new pool, add `amount_a` and `amount_b` as is
    //     // (already limited by depositor account balances above)
    //   } else {
    //     // Calculate the pool ratio to maintain proper liquidity proportions
    //     const ratio = pool_account_a_amount.mul(pool_account_b_amount);
    //     if (pool_account_a_amount.gt(pool_account_b_amount)) {
    //       amount_a = amount_b.mul(ratio).toNum();
    //     } else {
    //       amount_b = amount_a.div(ratio).toNum();
    //     }
    //   }

    // Computing the amount of liquidity about to be deposited
    // let liquidity = new i64(amount_a)
    // .mul(new i64(amount_b))
    // .sqrt()

    // if pool_creation {
    //     if liquidity < MINIMUM_LIQUIDITY {
    //         return new PoseidonError("DepositTooSmall");
    //     }

    //     liquidity -= MINIMUM_LIQUIDITY;
    // }

    // let liquidity = amount_a.mul(Number(amount_b));

    // Transfer tokens to the pool;
    // TokenProgram.transfer(
    //   depositor_account_a,
    //   pool_account_a,
    //   depositor,
    //   amount_a
    // );

    // TokenProgram.transfer(
    //   depositor_account_b,
    //   pool_account_b,
    //   depositor,
    //   amount_b
    // );

    // // mint the liquidity to the user
    // TokenProgram.mintTo(
    //   mint_liquidity,
    //   depositor_account_liquidity,
    //   pool_authority,
    //   liquidity
    // );
  }

  swap_exact_tokens_for_tokens(
    payer: Signer,
    trader: Signer,
    pool: Pool,
    pool_authority: PoolAuthority,
    pool_account_a: AssociatedTokenAccount,
    pool_account_b: AssociatedTokenAccount,
    trader_account_a: AssociatedTokenAccount,
    trader_account_b: AssociatedTokenAccount,
    amm: AMM,
    mint_a: Mint,
    mint_b: Mint,
    fee: u16,
    amount_a: u64,
    amount_b: u64,
    // swap_a:bool
    input_amount: u64,
    min_input_amount: u64,
    id: u64,
  ) {
    amm.derive([id.toBytes()]);
    pool.derive([amm.key, mint_a.key, mint_b.key]).has([amm, mint_a, mint_b]);
    pool_authority.derive([amm.key, mint_a.key, mint_b.key, 'authority']);
    pool_account_a.derive(mint_a, pool_authority.key).init();
    pool_account_b.derive(mint_b, pool_authority.key).init();
    trader_account_a.derive(mint_a, trader.key).init();
    trader_account_b.derive(mint_b, trader.key).init();

    // Prevent depositing assets the depositor does not own
    // let input;
    // if (swap_a && input_amount.gt(trader_account_a.amount)) {
    //   input = trader_account_a.amount;
    // } else if (!swap_a && input_amount.gt(trader_account_b.amount)) {
    //   input = trader_account_b.amount;
    // } else {
    //   input = input_amount;
    // }

    // // Apply trading fee, used to compute the output
    // const taxed_input = input.sub(input.mul(amm.fee).div(new i64(10000)));

    // // Define pool accounts
    // const pool_a = pool_account_a;
    // const pool_b = pool_account_b;

    // // Calculate output based on the pool and trading direction
    // let output;
    // if (swap_a) {
    //   output = taxed_input
    //     .mul(pool_b.amount)
    //     .div(pool_a.amount.add(taxed_input))
    //     .toNum();
    // } else {
    //   output = taxed_input
    //     .mul(pool_a.amount)
    //     .div(pool_b.amount.add(taxed_input))
    //     .toNum();
    // }

    // // Ensure output is greater than the minimum required output
    // if (output.lt(min_output_amount)) {
    //   throw new Error("OutputTooSmall");
    // }

    // // Compute the invariant before the trade
    // const invariant = pool_a.amount.mul(pool_b.amount);

    //Transfer tokens to the pool

    // if (swap_a) {
    //      TokenProgram.transfer(
    //          trader_account_a,
    //          pool_account_a,
    //          trader,
    //          input
    //      )
    //      TokenProgram.transfer(
    //          pool_account_a,
    //          trader_account_a,
    //          pool_authority,
    //          output
    //      )
    // } else {
    //      TokenProgram.transfer(
    //          pool_account_a,
    //          trader_account_a,
    //          pool_authority,
    //          input
    //      )
    //      TokenProgram.transfer(
    //          trader_account_b,
    //          pool_account_b,
    //          trader,
    //          output
    //      )
    // }

    // Verify the invariant still holds
    // Reload accounts because of the CPIs
    // We tolerate if the new invariant is higher because it means a rounding error for LPs
    //pool_account_a.reload()
    //pool_account_b.reload()

    //  if invariant > pool_account_a.amount.mul(pool_account_a.amount) {
    //     return new PoseidonErr("Invariant Violated");
    // }
  }
  withdraw_liquidity(
    payer: Signer,
    depositor: Signer,
    pool: Pool,
    pool_authority: PoolAuthority,
    pool_account_a: AssociatedTokenAccount,
    pool_account_b: AssociatedTokenAccount,
    depositor_account_a: AssociatedTokenAccount,
    depositor_account_b: AssociatedTokenAccount,
    depositor_account_liquidity: AssociatedTokenAccount,
    amm: AMM,
    mint_liquidity: Mint,
    mint_a: Mint,
    mint_b: Mint,
    amount: u64,
    id: u64,
  ) {
    amm.derive([id.toBytes()]);
    pool.derive([amm.key, mint_a.key, mint_b.key]).has([mint_a, mint_b]);
    pool_authority.derive([amm.key, mint_a.key, mint_b.key, 'authority']);
    pool_account_a.derive(mint_a, pool_authority.key).init();
    pool_account_b.derive(mint_b, pool_authority.key).init();

    pool_account_a.derive(mint_a, pool_authority.key).init();
    pool_account_b.derive(mint_b, pool_authority.key).init();

    depositor_account_liquidity.derive(mint_liquidity, depositor.key).init();

    depositor_account_a.derive(mint_a, depositor.key).initIfNeeded();
    depositor_account_b.derive(mint_b, depositor.key).initIfNeeded();

    // let MINIMUM_LIQUIDITY = 1;

    // Transfer tokens from the pool
    // let amount_a = new i64(amount)
    //   .mul(new i64(pool_account_a.amount))
    //   .div(new i64(mint_liquidity.supply.add(MINIMUM_LIQUIDITY)))
    //   .floor()
    //   .toNum();

    // TokenProgram.transfer(
    //     pool_account_a,
    //     depositor_account_a,
    //     pool_authority,
    //     amount_a
    // )

    // let amount_b = new i64(amount)
    //   .mul(new i64(pool_account_b.amount))
    //   .div(new i64(mint_liquidity.supply.add(MINIMUM_LIQUIDITY)))
    //   .floor()
    //   .toNum();

    // TokenProgram.transfer(
    //     pool_account_b,
    //     depositor_account_b,
    //     pool_authority,
    //     amount_b
    // );

    // TokenProgram.burn(
    //   mint_liquidity,
    //   depositor_account_liquidity,
    //   depositor,
    //   amount
    // );
  }
}

export interface AMM extends Account {
  /// The primary key of the AMM
  id: u64;

  /// Account that has admin authority over the AMM
  admin: Pubkey;

  /// The LP fee taken on each trade, in basis points
  fee: u16;
}

export interface Pool extends Account {
  /// Primary key of the AMM
  amm: Pubkey;

  /// Mint of token A
  mint_a: Pubkey;

  /// Mint of token B
  mint_b: Pubkey;
}

// The admin of the AMM
//Read only delegatable creation
export interface Admin extends Account {}

//Read only authority
export interface PoolAuthority extends Account {}
