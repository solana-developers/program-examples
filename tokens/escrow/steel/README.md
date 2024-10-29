# Escrow

**Escrow** is a an example of an escrow holding tokens on behalf of a user.

## API

- [`Consts`](api/src/consts.rs) – Program constants.
- [`Error`](api/src/error.rs) – Custom program errors.
- [`Instruction`](api/src/instruction.rs) – Declared instructions.

## Instructions

- [`MakeOffer`](program/src/make_offer.rs) Makes an offer ...
- [`TakeOfferr`](program/src/take_offer.rs) Takes an offer ...
- [`Refund`](program/src/refund.rs) Refunds an offer to the ochestrator ...

## State

- [`Offer`](api/src/state/offer.rs) – Offer ...

## Get started

Compile your program:

```sh
steel build
```

Install dependencies:

```sh
pnpm install
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
