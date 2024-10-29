# CloseAccount

**CloseAccount** is a example program show you how to close account
        
## API
- [`Consts`](api/src/consts.rs) – Program constants.
- [`Error`](api/src/error.rs) – Custom program errors.
- [`Instruction`](api/src/instruction.rs) – Declared instructions.

## Instructions
- [`CreateUser`](program/src/create_user.rs) – Create user state account ...
- [`CloseUser`](program/src/close_user.rs) – Close user state account ...

## State
- [`UserState`](api/src/state/user_state.rs) – Counter ...

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
