# PDA Rent Payer

**PDA Rent Payer** is a program that uses a PDA to pay the rent
for the creation of a system program by simply transferring lamports to it
        
## API
- [`Consts`](api/src/consts.rs) – Program constants.
- [`Error`](api/src/error.rs) – Custom program errors.
- [`Instruction`](api/src/instruction.rs) – Declared instructions.

## Instructions
- [`Add`](program/src/add.rs) – Add ...
- [`Initialize`](program/src/initialize.rs) – Initialize ...

## State
- [`Counter`](api/src/state/counter.rs) – Counter ...

## Get started

Compile your program:
```sh
steel build
```

Run unit and integration tests:
```sh
steel test
```
