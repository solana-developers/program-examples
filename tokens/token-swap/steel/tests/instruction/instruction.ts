export class Assignable {
  constructor(properties) {
    for (const [key, value] of Object.entries(properties)) {
      this[key] = value;
    }
  }
}

export enum TokenSwapInstruction {
  CreateAmm = 0,
  CreatePool = 1,
  DepositLiquidity = 2,
  SwapExactTokensForTokens = 3,
  WithdrawLiquidity = 4,
}
