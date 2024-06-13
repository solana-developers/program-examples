# Program Examples

## Onchain program examples for :anchor: Anchor :crab: Native Rust, :snake: Python and Solidity

This repo contains Solana onchain programs (referred to as 'Smart Contracts' in other blockchains).

> [!NOTE]
> If you're new to Solana, you don't need to create your own programs to perform basic things like making accounts, creating tokens, sending tokens, or minting NFTs. These common tasks are handled with existing programs, for example the System Program (for making account or transferring SOL) or the token program (for creating tokens and NFTs). See the [Solana Developer site](https://solana.com/developers) to learn more.

## Using this repo

Each folder includes examples for one or more of the following

- `anchor` - Written using [Anchor](https://www.anchor-lang.com/), the most popular framework for Solana Development, which uses Rust. Use `anchor build && anchor deploy` to build & deploy the program. Run `anchor run test` to test it.
- `native` - Written using Solana's native Rust crates and vanilla Rust. Use `cicd.sh` to build & deploy the program. Run `yarn run test` to test it.
- `seahorse` - Written using the [Seahorse framework](https://seahorse-lang.org/), which converts your Python code to Anchor Rust. Use `seahorse build && anchor deploy` to build & deploy the program. Run `anchor run test` to test it.
- `solidity` - Written using Solidity.

If a given example is missing, please add it!

## The example projects!

### basics

#### account-data

[anchor](./basics/account-data/anchor), [native](./basics/account-data/native)

#### checking-accounts

[anchor](./basics/checking-accounts/anchor), [native](./basics/checking-accounts/native)

#### close-account

[anchor](./basics/close-account/anchor), [native](./basics/close-account/native)

#### counter

[anchor](./basics/counter/anchor), [native](./basics/counter/native), [seahorse](./basics/counter/seahorse)

#### create-account

[anchor](./basics/create-account/anchor), [native](./basics/create-account/native)

#### cross-program-invocation

[anchor](./basics/cross-program-invocation/anchor), [native](./basics/cross-program-invocation/native)

#### favorites

[anchor](./basics/favorites/anchor)

#### hello-solana

[anchor](./basics/hello-solana/anchor), [native](./basics/hello-solana/native), [seahorse](./basics/hello-solana/seahorse)

#### pda-rent-payer

[anchor](./basics/pda-rent-payer/anchor), [native](./basics/pda-rent-payer/native)

#### processing-instructions

[anchor](./basics/processing-instructions/anchor), [native](./basics/processing-instructions/native)

#### program-derived-addresses

[anchor](./basics/program-derived-addresses/anchor), [native](./basics/program-derived-addresses/native)

#### realloc

[anchor](./basics/realloc/anchor), [native](./basics/realloc/native)

#### rent

[anchor](./basics/rent/anchor), [native](./basics/rent/native)

#### repository-layout

[anchor](./basics/repository-layout/anchor), [native](./basics/repository-layout/native)

#### transfer-sol

[anchor](./basics/transfer-sol/anchor), [native](./basics/transfer-sol/native), [seahorse](./basics/transfer-sol/seahorse)

### tokens

#### create-token

[anchor](./tokens/create-token/anchor), [native](./tokens/create-token/native)

#### escrow

[anchor](./tokens/escrow/anchor)

#### nft-minter

[anchor](./tokens/nft-minter/anchor), [native](./tokens/nft-minter/native)

#### pda-mint-authority

[anchor](./tokens/pda-mint-authority/anchor), [native](./tokens/pda-mint-authority/native)

#### spl-token-minter

[anchor](./tokens/spl-token-minter/anchor), [native](./tokens/spl-token-minter/native)

#### token-swap

[anchor](./tokens/token-swap/anchor)

#### transfer-tokens

[anchor](./tokens/transfer-tokens/anchor), [native](./tokens/transfer-tokens/native), [seahorse](./tokens/transfer-tokens/seahorse)

### tokens/token-2022

#### basics

[anchor](./tokens/token-2022/basics/anchor)

#### cpi-guard

[anchor](./tokens/token-2022/cpi-guard/anchor)

#### default-account-state

[anchor](./tokens/token-2022/default-account-state/anchor), [native](./tokens/token-2022/default-account-state/native)

#### group

[anchor](./tokens/token-2022/group/anchor)

#### immutable-owner

[anchor](./tokens/token-2022/immutable-owner/anchor)

#### interest-bearing

[anchor](./tokens/token-2022/interest-bearing/anchor)

#### memo-transfer

[anchor](./tokens/token-2022/memo-transfer/anchor)

#### metadata

[anchor](./tokens/token-2022/metadata/anchor)

#### mint-close-authority

[anchor](./tokens/token-2022/mint-close-authority/anchor), [native](./tokens/token-2022/mint-close-authority/native)

#### multiple-extensions

[native](./tokens/token-2022/multiple-extensions/native)

#### non-transferable

[anchor](./tokens/token-2022/non-transferable/anchor), [native](./tokens/token-2022/non-transferable/native)

#### permanent-delegate

[anchor](./tokens/token-2022/permanent-delegate/anchor)

#### transfer-fee

[anchor](./tokens/token-2022/transfer-fee/anchor), [native](./tokens/token-2022/transfer-fee/native)

### compression

#### cnft-burn

[anchor](./compression/cnft-burn/anchor)

#### cnft-vault

[anchor](./compression/cnft-vault/anchor)

#### cutils

[anchor](./compression/cutils/anchor)

### oracles

#### pyth

[anchor](./oracles/pyth/anchor), [seahorse](./oracles/pyth/seahorse)

## Examples We'd Love to See!

- Examples needed for Native:
  - Token Extensions
- Examples needed for Anchor:
  - Additional Accounts & Resolving Accounts
- Examples needed for Seahorse
  - Any existing example missing a `seahorse` folder
- New examples needed for Anchor, Native, Solidity & Seahorse:
  - Token lending
  - Token swapping
  - Escrow
  - Staking
  - Wrapped tokens
  - Pyth
  - Clockwork
  - VRF
  - Any oracle
  - Merkle trees (compression)

---
