# AccountDataProgram

**AccountDataProgram** is a program that allows you to store and retrieve data from a Solana account.

## API

- [`Consts`](api/src/consts.rs) – Program constants.
- [`Error`](api/src/error.rs) – Custom program errors.
- [`Instruction`](api/src/instruction.rs) – Declared instructions.

## Instructions

- [`Initialize`](program/src/initialize.rs) – Initialize the data account

## State

- [`AddressInfoData`](api/src/state/address_info.rs) – Account data structure.

## How to?

Compile your program:

```sh
pnpm build
```

Run tests:

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
