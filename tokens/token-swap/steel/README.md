# Token swap example amm in Steel

**TokenSwap** - Your Gateway to Effortless Trading! Welcome to the world of Automated Market Makers (AMM), where seamless trading is made possible with the power of automation. The primary goal of AMMs is to act as automatic buyers and sellers, readily available whenever users wish to trade their assets.
        
## API
- [`Consts`](api/src/consts.rs) – Program constants.
- [`Error`](api/src/error.rs) – Custom program errors.
- [`Instruction`](api/src/instruction.rs) – Declared instructions.

## Instructions
- [`CreateAmm`](program/src/create_amm.rs) – Create amm ...
- [`CreatePool`](program/src/create_pool.rs) – Create liquidity pool
- [`DepositLiquidity`](program/src/deposit_liquidity.rs) – Desposit liquidity to pool
- [`WithdrawLiquidity`](program/src/withdraw_liquidity.rs) – Withdraw liquidity from pool
- [`Swap`](program/src/swap.rs) – Swap exact token amount

## State
- [`Amm`](api/src/state/amm.rs) – Amm state
- [`Pool`](api/src/state/pool.rs) – Pool state

## How to run

Compile your program:

```sh
pnpm build
```

Run unit and integration tests:

```sh
pnpm test
```

Run build and test

```sh
pnpm build-and-test
```

Deploy your program:

```sh
pnpm deploy
```
