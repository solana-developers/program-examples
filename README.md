# Program Examples

## Onchain program examples for ⚓ Anchor, 🤥 Pinocchio, 💫 Quasar, and 🦀 Native Rust.

[![Anchor](https://github.com/solana-developers/program-examples/actions/workflows/anchor.yml/badge.svg?event=schedule)](https://github.com/solana-developers/program-examples/actions/workflows/anchor.yml) [![Pinocchio](https://github.com/solana-developers/program-examples/actions/workflows/solana-pinocchio.yml/badge.svg?event=schedule)](https://github.com/solana-developers/program-examples/actions/workflows/solana-pinocchio.yml) [![Native](https://github.com/solana-developers/program-examples/actions/workflows/solana-native.yml/badge.svg?event=schedule)](https://github.com/solana-developers/program-examples/actions/workflows/solana-native.yml) [![Quasar](https://github.com/solana-developers/program-examples/actions/workflows/solana-quasar.yml/badge.svg?event=schedule)](https://github.com/solana-developers/program-examples/actions/workflows/solana-quasar.yml)

This repo contains Solana onchain programs (referred to as 'Smart Contracts' in other blockchains).

> [!NOTE]
> If you're new to Solana, you don't need to create your own programs to perform basic things like making accounts, creating tokens, sending tokens, or minting NFTs. These common tasks are handled with existing programs, for example the System Program (for making account or transferring SOL) or the token program (for creating tokens and NFTs). See the [Solana Developer site](https://solana.com/developers) to learn more.

> ⚠️ This repository uses **pnpm** as the default package manager.  
> Ensure pnpm is installed before running any examples.

Each folder includes examples for one or more of the following:

- `anchor` - Written using [Anchor](https://www.anchor-lang.com/), the most popular framework for Solana development, which uses Rust.
  Use `anchor build` and `anchor deploy` to build and deploy the program.
  Tests should be executed using `pnpm test` as defined in the `Anchor.toml` scripts section.

- `pinocchio` - Written using [Pinocchio](https://github.com/febo/pinocchio), a zero-copy, zero-allocation library for Solana programs.
  Build and test commands are the same as native examples.
  Run `pnpm test` to execute tests.

- `quasar` - Written using [Quasar](https://github.com/blueshift-gg/quasar), a zero-copy, zero-allocation `no_std` framework for Solana programs with Anchor-compatible ergonomics.
  Build and test commands are the same as native examples.
  Run `pnpm test` to execute tests.

- `native` - Written using Solana's native Rust crates and vanilla Rust.
  Build and test commands are defined via pnpm scripts and use `litesvm` for testing.
  Run `pnpm test` to execute tests.


**If a given example is missing, please send us a PR to add it!** Our aim is to have every example available in every option. We'd also love to see more programs involving staking, wrapped tokens, oracles, compression and VRF. Follow the [contributing guidelines](./CONTRIBUTING.md) to keep things consistent.

## The example programs
## Basics
### Hello world

[Hello World on Solana! A minimal program that logs a greeting.](./basics/hello-solana/README.md)

[anchor](./basics/hello-solana/anchor) [pinocchio](./basics/hello-solana/pinocchio) [quasar](./basics/hello-solana/quasar) [native](./basics/hello-solana/native)

### Account-data

Store and retrieve data using Solana accounts.

[anchor](./basics/account-data/anchor) [pinocchio](./basics/account-data/pinocchio) [quasar](./basics/account-data/quasar) [native](./basics/account-data/native)

### Storing global state - Counter

[Use a PDA to store global state, making a counter that increments when called.](./basics/counter/README.md)

[anchor](./basics/counter/anchor) [pinocchio](./basics/counter/pinocchio) [quasar](./basics/counter/quasar) [native](./basics/counter/native)

### Saving per-user state - Favorites

Save and update per-user state on the blockchain, ensuring users can only update their own information.

[anchor](./basics/favorites/anchor) [pinocchio](./basics/favorites/pinocchio) [quasar](./basics/favorites/quasar) [native](./basics/favorites/native)

### Checking Instruction Accounts

[Check that the accounts provided in incoming instructions meet particular criteria.](./basics/checking-accounts/README.md)

[anchor](./basics/checking-accounts/anchor) [pinocchio](./basics/checking-accounts/pinocchio) [quasar](./basics/checking-accounts/quasar) [native](./basics/checking-accounts/native)

### Closing Accounts

Close an account and get the Lamports back.

[anchor](./basics/close-account/anchor) [pinocchio](./basics/close-account/pinocchio) [quasar](./basics/close-account/quasar) [native](./basics/close-account/native)

### Creating Accounts

[Make new accounts on the blockchain.](./basics/create-account/README.md)

[anchor](./basics/create-account/anchor) [pinocchio](./basics/create-account/pinocchio) [quasar](./basics/create-account/quasar) [native](./basics/create-account/native)

### Cross program invocations

[Invoke an instruction handler from one onchain program in another onchain program.](./basics/cross-program-invocation/README.md)

[anchor](./basics/cross-program-invocation/anchor) [quasar](./basics/cross-program-invocation/quasar) [native](./basics/cross-program-invocation/native)

### PDA rent-payer

[Use a PDA to pay the rent for the creation of a new account.](./basics/pda-rent-payer/README.md)

[anchor](./basics/pda-rent-payer/anchor) [pinocchio](./basics/pda-rent-payer/pinocchio) [quasar](./basics/pda-rent-payer/quasar) [native](./basics/pda-rent-payer/native)

### Processing instructions

[Add parameters to an instruction handler and use them.](./basics/processing-instructions/README.md)

[anchor](./basics/processing-instructions/anchor) [pinocchio](./basics/processing-instructions/pinocchio) [quasar](./basics/processing-instructions/quasar) [native](./basics/processing-instructions/native)

### Storing date in program derived addresses

Store and retrieve state in Solana.

[anchor](./basics/program-derived-addresses/anchor) [pinocchio](./basics/program-derived-addresses/pinocchio) [quasar](./basics/program-derived-addresses/quasar) [native](./basics/program-derived-addresses/native)

### Handling accounts that expand in size

How to store state that changes size in Solana.

[anchor](./basics/realloc/anchor) [pinocchio](./basics/realloc/pinocchio) [quasar](./basics/realloc/quasar) [native](./basics/realloc/native)

### Calculating account size to determine rent

[Determine the necessary minimum rent by calculating an account's size.](./basics/rent/README.md)

[anchor](./basics/rent/anchor) [pinocchio](./basics/rent/pinocchio) [quasar](./basics/rent/quasar) [native](./basics/rent/native)

### Laying out larger programs

[Layout larger Solana onchain programs.](./basics/repository-layout/README.md)

[anchor](./basics/repository-layout/anchor) [quasar](./basics/repository-layout/quasar) [native](./basics/repository-layout/native)

### Transferring SOL

[Send SOL between two accounts.](./basics/transfer-sol/README.md)

[anchor](./basics/transfer-sol/anchor) [pinocchio](./basics/transfer-sol/pinocchio) [quasar](./basics/transfer-sol/quasar) [native](./basics/transfer-sol/native)
## Tokens
### Creating tokens

[Create a token on Solana with a token symbol and icon.](./tokens/create-token/README.md)

[anchor](./tokens/create-token/anchor) [quasar](./tokens/create-token/quasar) [native](./tokens/create-token/native)

### Minting NFTS

[Mint an NFT from inside your own onchain program using the Token and Metaplex Token Metadata programs.](./tokens/nft-minter/README.md) Reminder: you don't need your own program just to mint an NFT, see the note at the top of this README.

[anchor](./tokens/nft-minter/anchor) [quasar](./tokens/nft-minter/quasar) [native](./tokens/nft-minter/native)

### NFT operations

Create an NFT collection, mint NFTs, and verify NFTs as part of a collection using Metaplex Token Metadata.

[anchor](./tokens/nft-operations/anchor) [quasar](./tokens/nft-operations/quasar)

### Minting a token from inside a program

[Mint a Token from inside your own onchain program using the Token program.](./tokens/spl-token-minter/README.md) Reminder: you don't need your own program just to mint an NFT, see the note at the top of this README.

[anchor](./tokens/spl-token-minter/anchor) [quasar](./tokens/spl-token-minter/quasar) [native](./tokens/spl-token-minter/native)

### Transferring Tokens

[Transfer tokens between accounts](./tokens/transfer-tokens/README.md)

[anchor](./tokens/transfer-tokens/anchor) [quasar](./tokens/transfer-tokens/quasar) [native](./tokens/transfer-tokens/native)

### Allowing users to swap digital assets - Escrow

Allow two users to swap digital assets with each other, each getting 100% of what the other has offered due to the power of decentralization!

[anchor](./tokens/escrow/anchor) [quasar](./tokens/escrow/quasar) [native](./tokens/escrow/native)

### Fundraising with SPL Tokens

Create a fundraiser account specifying a target mint and amount, allowing contributors to deposit tokens until the goal is reached.

[anchor](./tokens/token-fundraiser/anchor) [quasar](./tokens/token-fundraiser/quasar)

### Minting a token from inside a program with a PDA as the mint authority

[Mint a Token from inside your own onchain program using the Token program.](./tokens/pda-mint-authority/README.md) Reminder: you don't need your own program just to mint an NFT, see the note at the top of this README.

[anchor](./tokens/pda-mint-authority/anchor) [quasar](./tokens/pda-mint-authority/quasar) [native](./tokens/pda-mint-authority/native)

### Creating an Automated Market Maker

[Create liquidity pools to allow trading of new digital assets and allows users that provide liquidity to be rewarded by creating an Automated Market Maker.](./tokens/token-swap/README.md)

[anchor](./tokens/token-swap/anchor) [quasar](./tokens/token-swap/quasar)

### External delegate token master

Control token transfers using an external secp256k1 delegate signature.

[anchor](./tokens/external-delegate-token-master/anchor) [quasar](./tokens/external-delegate-token-master/quasar)
## Token Extensions
### Basics - create token mints, mint tokens, and transfer tokens with Token Extensions

Create token mints, mint tokens, and transfer tokens using Token Extensions.

[anchor](./tokens/token-2022/basics/anchor) [quasar](./tokens/token-2022/basics/quasar)

### Preventing CPIs with CPI guard

Enable CPI guard to prevents certain token action from occurring within CPI (Cross-Program Invocation).

[anchor](./tokens/token-2022/cpi-guard/anchor) [quasar](./tokens/token-2022/cpi-guard/quasar)

### Using default account state

Create new token accounts that are frozen by default.

[anchor](./tokens/token-2022/default-account-state/anchor) [quasar](./tokens/token-2022/default-account-state/quasar) [native](./tokens/token-2022/default-account-state/native)

### Grouping tokens

Create tokens that belong to larger groups of tokens using the Group Pointer extension.

[anchor](./tokens/token-2022/group/anchor) [quasar](./tokens/token-2022/group/quasar)

### Creating token accounts whose owner cannot be changed

Create tokens whose owning program cannot be changed.

[anchor](./tokens/token-2022/immutable-owner/anchor) [quasar](./tokens/token-2022/immutable-owner/quasar)

### Interest bearing tokens

Create tokens that show an 'interest' calculation.

[anchor](./tokens/token-2022/interest-bearing/anchor) [quasar](./tokens/token-2022/interest-bearing/quasar)

### Requiring transactions to include descriptive memos

Create tokens where transfers must have a memo describing the transaction attached.

[anchor](./tokens/token-2022/memo-transfer/anchor) [quasar](./tokens/token-2022/memo-transfer/quasar)

### Adding on-chain metadata to the token mint

Create tokens that store their onchain metadata inside the token mint, without needing to use or pay for additional programs.

[anchor](./tokens/token-2022/metadata/anchor) [quasar](./tokens/token-2022/metadata/quasar)

### Storing NFT metadata using the metadata pointer extension

Create an NFT using the Token Extensions metadata pointer, storing onchain metadata (including custom fields) inside the mint account itself.

[anchor](./tokens/token-2022/nft-meta-data-pointer/anchor-example/anchor)

### Allow a designated account to close a mint

Allow a designated account to close a Mint.

[anchor](./tokens/token-2022/mint-close-authority/anchor) [quasar](./tokens/token-2022/mint-close-authority/quasar) [native](./tokens/token-2022/mint-close-authority/native)

### Using multiple token extensions

Use multiple Token Extensions at once.

[native](./tokens/token-2022/multiple-extensions/native)

### Non-transferrable - create tokens that can't be transferred.

Create tokens that cannot be transferred.

[anchor](./tokens/token-2022/non-transferable/anchor) [quasar](./tokens/token-2022/non-transferable/quasar) [native](./tokens/token-2022/non-transferable/native)

### Permanent Delegate - Create tokens permanently under the control of a particular account

Create tokens that remain under the control of an account, even when transferred elsewhere.

[anchor](./tokens/token-2022/permanent-delegate/anchor) [quasar](./tokens/token-2022/permanent-delegate/quasar)

### Create tokens with a transfer-fee.

Create tokens with an inbuilt transfer fee.

[anchor](./tokens/token-2022/transfer-fee/anchor) [quasar](./tokens/token-2022/transfer-fee/quasar) [native](./tokens/token-2022/transfer-fee/native)

### Transfer hook - hello world

A minimal transfer hook program that executes custom logic on every token transfer.

[anchor](./tokens/token-2022/transfer-hook/hello-world/anchor) [quasar](./tokens/token-2022/transfer-hook/hello-world/quasar)

### Transfer hook - counter

Count how many times tokens have been transferred using a transfer hook.

[anchor](./tokens/token-2022/transfer-hook/counter/anchor) [quasar](./tokens/token-2022/transfer-hook/counter/quasar)

### Transfer hook - using account data as seed

Use token account owner data as seeds to derive extra accounts in a transfer hook.

[anchor](./tokens/token-2022/transfer-hook/account-data-as-seed/anchor) [quasar](./tokens/token-2022/transfer-hook/account-data-as-seed/quasar)

### Transfer hook - allow/block list

Restrict or allow token transfers using an on-chain allow/block list managed by a list authority.

[anchor](./tokens/token-2022/transfer-hook/allow-block-list-token/anchor) [quasar](./tokens/token-2022/transfer-hook/allow-block-list-token/quasar)

### Transfer hook - transfer cost

Charge an additional cost or fee on every token transfer using a transfer hook.

[anchor](./tokens/token-2022/transfer-hook/transfer-cost/anchor) [quasar](./tokens/token-2022/transfer-hook/transfer-cost/quasar)

### Transfer hook - transfer switch

Enable or disable token transfers with an on-chain switch using a transfer hook.

[anchor](./tokens/token-2022/transfer-hook/transfer-switch/anchor) [quasar](./tokens/token-2022/transfer-hook/transfer-switch/quasar)

### Transfer hook - whitelist

Restrict token transfers so only whitelisted accounts can receive tokens.

[anchor](./tokens/token-2022/transfer-hook/whitelist/anchor) [quasar](./tokens/token-2022/transfer-hook/whitelist/quasar)
## Compression
### Cnft-burn

Burn compressed NFTs.

[anchor](./compression/cnft-burn/anchor) [quasar](./compression/cnft-burn/quasar)

### Cnft-vault

Store Metaplex compressed NFTs inside a PDA.

[anchor](./compression/cnft-vault/anchor) [quasar](./compression/cnft-vault/quasar)

### Cutils

Work with Metaplex compressed NFTs.

[anchor](./compression/cutils/anchor) [quasar](./compression/cutils/quasar)
## Oracles
### pyth

Use a data source for offchain data (called an Oracle) to perform activities onchain.

[anchor](./oracles/pyth/anchor) [quasar](./oracles/pyth/quasar)
## Tools
### Shank and Solita

Use Shank and Solita to generate IDLs and TypeScript clients for native Solana programs, the same way Anchor does for Anchor programs.

[native](./tools/shank-and-solita/native)

---
