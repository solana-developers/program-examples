# Solana Program Examples

## Solana onchain program examples for ⚓ Anchor, 💫 Quasar, 🤥 Pinocchio, and 🦀 Native Rust.

[![Anchor](https://github.com/solana-developers/program-examples/actions/workflows/anchor.yml/badge.svg?event=schedule)](https://github.com/solana-developers/program-examples/actions/workflows/anchor.yml) [![Quasar](https://github.com/solana-developers/program-examples/actions/workflows/solana-quasar.yml/badge.svg?event=schedule)](https://github.com/solana-developers/program-examples/actions/workflows/solana-quasar.yml) [![Pinocchio](https://github.com/solana-developers/program-examples/actions/workflows/solana-pinocchio.yml/badge.svg?event=schedule)](https://github.com/solana-developers/program-examples/actions/workflows/solana-pinocchio.yml) [![Native](https://github.com/solana-developers/program-examples/actions/workflows/solana-native.yml/badge.svg?event=schedule)](https://github.com/solana-developers/program-examples/actions/workflows/solana-native.yml)

This repo contains Solana onchain programs (referred to as 'Smart Contracts' in other blockchains).

> [!NOTE]
> If you're new to Solana, you don't need to create your own programs to perform basic things like making accounts, creating tokens, sending tokens, or minting NFTs. These common tasks are handled with existing programs, for example the System Program (for making account or transferring SOL) or the token program (for creating tokens and NFTs). See the [Solana Developer site](https://solana.com/developers) to learn more.

Each folder includes examples for one or more of the following:

- `anchor` - Written using [Anchor](https://www.anchor-lang.com/), the most popular framework for Solana development, which uses Rust.
  Use `anchor build` and `anchor deploy` to build and deploy the program.
  Tests should be executed using `pnpm test` as defined in the `Anchor.toml` scripts section.

- `quasar` - Written using [Quasar](https://github.com/blueshift-gg/quasar), a zero-copy, zero-allocation `no_std` framework for Solana programs with Anchor-compatible ergonomics.
  Build and test commands are the same as native examples.
  Run `pnpm test` to execute tests.

- `pinocchio` - Written using [Pinocchio](https://github.com/febo/pinocchio), a zero-copy, zero-allocation library for Solana programs.
  Build and test commands are the same as native examples.
  Run `pnpm test` to execute tests.

- `native` - Written using Solana's native Rust crates and vanilla Rust.
  Build and test commands are defined via pnpm scripts and use `litesvm` for testing.
  Run `pnpm test` to execute tests.


**If a given example is missing, please send us a PR to add it!** Our aim is to have every example available in every option. We'd also love to see more programs involving staking, wrapped tokens, oracles, compression and VRF. Follow the [contributing guidelines](./CONTRIBUTING.md) to keep things consistent.

## The example programs
## Basics
### Hello world

Hello World on Solana! A minimal program that logs a greeting.

[Anchor](./basics/hello-solana/anchor) [Quasar](./basics/hello-solana/quasar) [Pinocchio](./basics/hello-solana/pinocchio) [Native](./basics/hello-solana/native)

### Account-data

Store and retrieve data using Solana accounts.

[Anchor](./basics/account-data/anchor) [Quasar](./basics/account-data/quasar) [Pinocchio](./basics/account-data/pinocchio) [Native](./basics/account-data/native)

### Storing global state - Counter

Use a PDA to store global state, making a counter that increments when called.

[Anchor](./basics/counter/anchor) [Quasar](./basics/counter/quasar) [Pinocchio](./basics/counter/pinocchio) [Native](./basics/counter/native)

### Saving per-user state - Favorites

Save and update per-user state on the blockchain, ensuring users can only update their own information.

[Anchor](./basics/favorites/anchor) [Quasar](./basics/favorites/quasar) [Pinocchio](./basics/favorites/pinocchio) [Native](./basics/favorites/native)

### Checking Instruction Accounts

Check that the accounts provided in incoming instructions meet particular criteria.

[Anchor](./basics/checking-accounts/anchor) [Quasar](./basics/checking-accounts/quasar) [Pinocchio](./basics/checking-accounts/pinocchio) [Native](./basics/checking-accounts/native)

### Closing Accounts

Close an account and get the Lamports back.

[Anchor](./basics/close-account/anchor) [Quasar](./basics/close-account/quasar) [Pinocchio](./basics/close-account/pinocchio) [Native](./basics/close-account/native)

### Creating Accounts

Make new accounts on the blockchain.

[Anchor](./basics/create-account/anchor) [Quasar](./basics/create-account/quasar) [Pinocchio](./basics/create-account/pinocchio) [Native](./basics/create-account/native)

### Cross program invocations

Invoke an instruction handler from one onchain program in another onchain program.

[Anchor](./basics/cross-program-invocation/anchor) [Quasar](./basics/cross-program-invocation/quasar) [Native](./basics/cross-program-invocation/native)

### PDA rent-payer

Use a PDA to pay the rent for the creation of a new account.

[Anchor](./basics/pda-rent-payer/anchor) [Quasar](./basics/pda-rent-payer/quasar) [Pinocchio](./basics/pda-rent-payer/pinocchio) [Native](./basics/pda-rent-payer/native)

### Processing instructions

Add parameters to an instruction handler and use them.

[Anchor](./basics/processing-instructions/anchor) [Quasar](./basics/processing-instructions/quasar) [Pinocchio](./basics/processing-instructions/pinocchio) [Native](./basics/processing-instructions/native)

### Storing date in program derived addresses

Store and retrieve state in Solana.

[Anchor](./basics/program-derived-addresses/anchor) [Quasar](./basics/program-derived-addresses/quasar) [Pinocchio](./basics/program-derived-addresses/pinocchio) [Native](./basics/program-derived-addresses/native)

### Handling accounts that expand in size

How to store state that changes size in Solana.

[Anchor](./basics/realloc/anchor) [Quasar](./basics/realloc/quasar) [Pinocchio](./basics/realloc/pinocchio) [Native](./basics/realloc/native)

### Calculating account size to determine rent

Determine the necessary minimum rent by calculating an account's size.

[Anchor](./basics/rent/anchor) [Quasar](./basics/rent/quasar) [Pinocchio](./basics/rent/pinocchio) [Native](./basics/rent/native)

### Laying out larger programs

Layout larger Solana onchain programs.

[Anchor](./basics/repository-layout/anchor) [Quasar](./basics/repository-layout/quasar) [Native](./basics/repository-layout/native)

### Transferring SOL

Send SOL between two accounts.

[Anchor](./basics/transfer-sol/anchor) [Quasar](./basics/transfer-sol/quasar) [Pinocchio](./basics/transfer-sol/pinocchio) [Native](./basics/transfer-sol/native)
## Tokens
### Creating tokens

Create a token on Solana with a token symbol and icon.

[Anchor](./tokens/create-token/anchor) [Quasar](./tokens/create-token/quasar) [Native](./tokens/create-token/native)

### Minting NFTS

Mint an NFT from inside your own onchain program using the Token and Metaplex Token Metadata programs. Reminder: you don't need your own program just to mint an NFT, see the note at the top of this README.

[Anchor](./tokens/nft-minter/anchor) [Quasar](./tokens/nft-minter/quasar) [Native](./tokens/nft-minter/native)

### NFT operations

Create an NFT collection, mint NFTs, and verify NFTs as part of a collection using Metaplex Token Metadata.

[Anchor](./tokens/nft-operations/anchor) [Quasar](./tokens/nft-operations/quasar)

### Minting a token from inside a program

Mint a Token from inside your own onchain program using the Token program. Reminder: you don't need your own program just to mint an NFT, see the note at the top of this README.

[Anchor](./tokens/spl-token-minter/anchor) [Quasar](./tokens/spl-token-minter/quasar) [Native](./tokens/spl-token-minter/native)

### Transferring Tokens

Transfer tokens between accounts

[Anchor](./tokens/transfer-tokens/anchor) [Quasar](./tokens/transfer-tokens/quasar) [Native](./tokens/transfer-tokens/native)

### Allowing users to swap digital assets - Escrow

Allow two users to swap digital assets with each other, each getting 100% of what the other has offered due to the power of decentralization!

[Anchor](./tokens/escrow/anchor) [Quasar](./tokens/escrow/quasar) [Native](./tokens/escrow/native)

### Fundraising with SPL Tokens

Create a fundraiser account specifying a target mint and amount, allowing contributors to deposit tokens until the goal is reached.

[Anchor](./tokens/token-fundraiser/anchor) [Quasar](./tokens/token-fundraiser/quasar)

### Minting a token from inside a program with a PDA as the mint authority

Mint a Token from inside your own onchain program using the Token program. Reminder: you don't need your own program just to mint an NFT, see the note at the top of this README.

[Anchor](./tokens/pda-mint-authority/anchor) [Quasar](./tokens/pda-mint-authority/quasar) [Native](./tokens/pda-mint-authority/native)

### Creating an Automated Market Maker

Create liquidity pools to allow trading of new digital assets and allows users that provide liquidity to be rewarded by creating an Automated Market Maker.

[Anchor](./tokens/token-swap/anchor) [Quasar](./tokens/token-swap/quasar)

### External delegate token master

Control token transfers using an external secp256k1 delegate signature.

[Anchor](./tokens/external-delegate-token-master/anchor) [Quasar](./tokens/external-delegate-token-master/quasar)
## Token Extensions
### Basics - create token mints, mint tokens, and transfer tokens with Token Extensions

Create token mints, mint tokens, and transfer tokens using Token Extensions.

[Anchor](./tokens/token-2022/basics/anchor) [Quasar](./tokens/token-2022/basics/quasar)

### Preventing CPIs with CPI guard

Enable CPI guard to prevents certain token action from occurring within CPI (Cross-Program Invocation).

[Anchor](./tokens/token-2022/cpi-guard/anchor) [Quasar](./tokens/token-2022/cpi-guard/quasar)

### Using default account state

Create new token accounts that are frozen by default.

[Anchor](./tokens/token-2022/default-account-state/anchor) [Quasar](./tokens/token-2022/default-account-state/quasar) [Native](./tokens/token-2022/default-account-state/native)

### Grouping tokens

Create tokens that belong to larger groups of tokens using the Group Pointer extension.

[Anchor](./tokens/token-2022/group/anchor) [Quasar](./tokens/token-2022/group/quasar)

### Creating token accounts whose owner cannot be changed

Create tokens whose owning program cannot be changed.

[Anchor](./tokens/token-2022/immutable-owner/anchor) [Quasar](./tokens/token-2022/immutable-owner/quasar)

### Interest bearing tokens

Create tokens that show an 'interest' calculation.

[Anchor](./tokens/token-2022/interest-bearing/anchor) [Quasar](./tokens/token-2022/interest-bearing/quasar)

### Requiring transactions to include descriptive memos

Create tokens where transfers must have a memo describing the transaction attached.

[Anchor](./tokens/token-2022/memo-transfer/anchor) [Quasar](./tokens/token-2022/memo-transfer/quasar)

### Adding on-chain metadata to the token mint

Create tokens that store their onchain metadata inside the token mint, without needing to use or pay for additional programs.

[Anchor](./tokens/token-2022/metadata/anchor) [Quasar](./tokens/token-2022/metadata/quasar)

### Storing NFT metadata using the metadata pointer extension

Create an NFT using the Token Extensions metadata pointer, storing onchain metadata (including custom fields) inside the mint account itself.

[Anchor](./tokens/token-2022/nft-meta-data-pointer/anchor-example/anchor)

### Allow a designated account to close a mint

Allow a designated account to close a Mint.

[Anchor](./tokens/token-2022/mint-close-authority/anchor) [Quasar](./tokens/token-2022/mint-close-authority/quasar) [Native](./tokens/token-2022/mint-close-authority/native)

### Using multiple token extensions

Use multiple Token Extensions at once.

[Native](./tokens/token-2022/multiple-extensions/native)

### Non-transferrable - create tokens that can't be transferred.

Create tokens that cannot be transferred.

[Anchor](./tokens/token-2022/non-transferable/anchor) [Quasar](./tokens/token-2022/non-transferable/quasar) [Native](./tokens/token-2022/non-transferable/native)

### Permanent Delegate - Create tokens permanently under the control of a particular account

Create tokens that remain under the control of an account, even when transferred elsewhere.

[Anchor](./tokens/token-2022/permanent-delegate/anchor) [Quasar](./tokens/token-2022/permanent-delegate/quasar)

### Create tokens with a transfer-fee.

Create tokens with an inbuilt transfer fee.

[Anchor](./tokens/token-2022/transfer-fee/anchor) [Quasar](./tokens/token-2022/transfer-fee/quasar) [Native](./tokens/token-2022/transfer-fee/native)

### Transfer hook - hello world

A minimal transfer hook program that executes custom logic on every token transfer.

[Anchor](./tokens/token-2022/transfer-hook/hello-world/anchor) [Quasar](./tokens/token-2022/transfer-hook/hello-world/quasar)

### Transfer hook - counter

Count how many times tokens have been transferred using a transfer hook.

[Anchor](./tokens/token-2022/transfer-hook/counter/anchor) [Quasar](./tokens/token-2022/transfer-hook/counter/quasar)

### Transfer hook - using account data as seed

Use token account owner data as seeds to derive extra accounts in a transfer hook.

[Anchor](./tokens/token-2022/transfer-hook/account-data-as-seed/anchor) [Quasar](./tokens/token-2022/transfer-hook/account-data-as-seed/quasar)

### Transfer hook - allow/block list

Restrict or allow token transfers using an on-chain allow/block list managed by a list authority.

[Anchor](./tokens/token-2022/transfer-hook/allow-block-list-token/anchor) [Quasar](./tokens/token-2022/transfer-hook/allow-block-list-token/quasar)

### Transfer hook - transfer cost

Charge an additional cost or fee on every token transfer using a transfer hook.

[Anchor](./tokens/token-2022/transfer-hook/transfer-cost/anchor) [Quasar](./tokens/token-2022/transfer-hook/transfer-cost/quasar)

### Transfer hook - transfer switch

Enable or disable token transfers with an on-chain switch using a transfer hook.

[Anchor](./tokens/token-2022/transfer-hook/transfer-switch/anchor) [Quasar](./tokens/token-2022/transfer-hook/transfer-switch/quasar)

### Transfer hook - whitelist

Restrict token transfers so only whitelisted accounts can receive tokens.

[Anchor](./tokens/token-2022/transfer-hook/whitelist/anchor) [Quasar](./tokens/token-2022/transfer-hook/whitelist/quasar)
## Compression
### Cnft-burn

Burn compressed NFTs.

[Anchor](./compression/cnft-burn/anchor) [Quasar](./compression/cnft-burn/quasar)

### Cnft-vault

Store Metaplex compressed NFTs inside a PDA.

[Anchor](./compression/cnft-vault/anchor) [Quasar](./compression/cnft-vault/quasar)

### Cutils

Work with Metaplex compressed NFTs.

[Anchor](./compression/cutils/anchor) [Quasar](./compression/cutils/quasar)
## Oracles
### pyth

Use a data source for offchain data (called an Oracle) to perform activities onchain.

[Anchor](./oracles/pyth/anchor) [Quasar](./oracles/pyth/quasar)
## Tools
### Shank and Solita

Use Shank and Solita to generate IDLs and TypeScript clients for native Solana programs, the same way Anchor does for Anchor programs.

[Native](./tools/shank-and-solita/native)

---
