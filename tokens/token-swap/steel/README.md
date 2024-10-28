# Token-Swap

**Token-Swap** is a an example of swapping tokens using an Automated Market Makers (AMM) style fashion.

## API

- [`Consts`](api/src/consts.rs) – Program constants.
- [`Error`](api/src/error.rs) – Custom program errors.
- [`Instruction`](api/src/instruction.rs) – Declared instructions.

## Instructions

- [`CreateAmm`](program/src/create_amm.rs) Create Amm ...
- [`CreatePool`](program/src/create_pool.rs) Create Pool ...
- [`DepositLiquidity`](program/src/deposit_liquidity.rs) Deposit Liquidity ...
- [`SwapExactTokens`](program/src/swap.rs) Swap Token A for B...
- [`WithdrawLiquidity`](program/src/withdraw.rs) Withdraw Liquidity ...

## State

- [`Amm`](api/src/state/amm.rs) – Amm ...
- [`Pool`](api/src/state/pool.rs) – Pool ...

## Get started

Compile your program:

```sh
steel build
```

Run unit and integration tests (native):

```sh
steel test
```

Run unit and integration tests (bankrun):

```sh
pnpm build-and-test
```

Run unit and integration tests without logs for a cleaner output (bankrun):

```sh
pnpm test-no-log
```
