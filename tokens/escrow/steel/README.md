# Escrow

**Escrow** is a ...
        
## API
- [`Consts`](api/src/consts.rs) – Program constants.
- [`Instruction`](api/src/instruction.rs) – Declared instructions.

## Instructions
- [`MakeOffer`](program/src/make_offer.rs) – Make an offer ...
- [`TakerOffer`](program/src/take_offer.rs) – Take an offer ...

## State
- [`Offer`](api/src/state/offer.rs) – Offer state ...

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
