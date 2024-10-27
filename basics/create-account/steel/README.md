# Steel: Create account

This example program is written using **Steel**, a framework for writing onchain programs
        
## API
- [`Consts`](api/src/consts.rs) – Program constants.
- [`Instruction`](api/src/instruction.rs) – Declared instructions.

## Instructions
- [`Initialize`](program/src/initialize.rs) – Initialize the account creation.

## State
- [`New Account`](api/src/state.rs) – Link account discriminator and the struct that stores unique user ID

## Get started

Compile your program:
```sh
pnpm build
```

Run unit and integration tests:
```sh
pnpm test
```

Do both together:
```sh
pnpm build-and-test
```
