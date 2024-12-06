export * from './create_amm';
export * from './create_pool';
export * from './deposit_liquidity';
export * from './swap_exact_tokens_for_tokens';
export * from './withdraw_liquidity';

export enum AmmInstruction {
    CreateAmm = 0,
    CreatePool = 1,
    DepositLiquidity = 2,
    SwapExactTokensForTokens = 3,
    WithdrawLiquidity = 4,
}

