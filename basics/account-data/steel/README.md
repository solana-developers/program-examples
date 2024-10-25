# AccountDataProgram

**AccountDataProgram** is a program that allows you to store and retrieve data from a Solana account.

## API

- [`Consts`](api/src/consts.rs) – Program constants.
- [`Error`](api/src/error.rs) – Custom program errors.
- [`Event`](api/src/event.rs) – Custom program events.
- [`Instruction`](api/src/instruction.rs) – Declared instructions.

## Instructions

- [`Initialize`](program/src/initialize.rs) – Initialize the data account

## State

- [`AddressInfoData`](api/src/state/address_info.rs) – Account data structure.

## Get started

Compile your program:

```sh
steel build
```

Run unit and integration tests:

```sh
steel test
```
