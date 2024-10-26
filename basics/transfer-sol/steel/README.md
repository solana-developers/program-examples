# TransferSol

**TransferSol** is a ...

## API

- [`Error`](api/src/error.rs) – Custom program errors.
- [`Instruction`](api/src/instruction.rs) – Declared instructions.

## Instructions

- [`TransferSolWithCpi`](program/src/transfer_sol_with_cpi.rs) – Invoke transfer SOL via CPI
- [`TransferSolWithProgram`](program/src/transfer_sol_with_program.rs) – Invoke transfer SOL via program

## How to run

Compile your program:

```sh
pnpm build
```

Run unit and integration tests:

```sh
steel test
```

or

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
