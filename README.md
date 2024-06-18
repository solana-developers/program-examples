# Program Examples

## Onchain program examples for :anchor: Anchor :crab: Native Rust, and :snake: Python

This repo contains Solana onchain programs (referred to as 'Smart Contracts' in other blockchains).

> [!NOTE]
> If you're new to Solana, you don't need to create your own programs to perform basic things like making accounts, creating tokens, sending tokens, or minting NFTs. These common tasks are handled with existing programs, for example the System Program (for making account or transferring SOL) or the token program (for creating tokens and NFTs). See the [Solana Developer site](https://solana.com/developers) to learn more.

## Using this repo

Each folder includes examples for one or more of the following

- `anchor` - Written using [Anchor](https://www.anchor-lang.com/), the most popular framework for Solana Development, which uses Rust. Use `anchor build && anchor deploy` to build & deploy the program. Run `anchor run test` to test it.
- `native` - Written using Solana's native Rust crates and vanilla Rust. Use `cicd.sh` to build & deploy the program. Run `yarn run test` to test it.
- `seahorse` - Written using the [Seahorse framework](https://seahorse-lang.org/), which converts your Python code to Anchor Rust. Use `seahorse build && anchor deploy` to build & deploy the program. Run `anchor run test` to test it.

If a given example is missing, please add it!

## The example projects

<details>
  <summary>Basics</summary>

#### account-data

Store and retrieve data using Solana accounts.

[anchor](./basics/account-data/anchor), [native](./basics/account-data/native)

#### checking-accounts

[Check that the accounts provided in incoming instructions meet particular criteria.](./basics/checking-accounts/README.md)

[anchor](./basics/checking-accounts/anchor), [native](./basics/checking-accounts/native)

#### close-account

Close an account and get the Lamports back.

[anchor](./basics/close-account/anchor), [native](./basics/close-account/native)

#### counter

[Use a PDA to store global state, making a counter that increments when called.](./basics/counter/README.md)

[anchor](./basics/counter/anchor), [native](./basics/counter/native), [seahorse](./basics/counter/seahorse)

#### create-account

[Make new accounts on the blockchain.](./basics/create-account/README.md)

[anchor](./basics/create-account/anchor), [native](./basics/create-account/native)

#### cross-program-invocation

[Invoke an instruction handler from one onchain program in another onchain program.](./basics/cross-program-invocation/README.md)

[anchor](./basics/cross-program-invocation/anchor), [native](./basics/cross-program-invocation/native)

#### favorites

Save and update per-user state on the blockchain, ensuring users can only update their own information.

[anchor](./basics/favorites/anchor)

#### hello-solana

[Hello World on Solana! A minimal program that logs a greeting.](./basics/hello-solana/README.md)

[anchor](./basics/hello-solana/anchor), [native](./basics/hello-solana/native), [seahorse](./basics/hello-solana/seahorse)

#### pda-rent-payer

[Use a PDA to pay the rent for the creation of a new account.](./basics/pda-rent-payer/README.md)

[anchor](./basics/pda-rent-payer/anchor), [native](./basics/pda-rent-payer/native)

#### processing-instructions

[Add parameters to an instruction handler and use them.](./basics/processing-instructions/README.md)

[anchor](./basics/processing-instructions/anchor), [native](./basics/processing-instructions/native)

#### program-derived-addresses

Store and retrieve state in Solana.

[anchor](./basics/program-derived-addresses/anchor), [native](./basics/program-derived-addresses/native)

#### realloc

How to store state that changes size in Solana.

[anchor](./basics/realloc/anchor), [native](./basics/realloc/native)

#### rent

[Determine the necessary minimum rent by calculating an account's size.](./basics/rent/README.md)

[anchor](./basics/rent/anchor), [native](./basics/rent/native)

#### repository-layout

[Layout larger Solana onchain programs.](./basics/repository-layout/README.md)

[anchor](./basics/repository-layout/anchor), [native](./basics/repository-layout/native)

#### transfer-sol

[Send SOL between two accounts.](./basics/transfer-sol/README.md)

[anchor](./basics/transfer-sol/anchor), [native](./basics/transfer-sol/native), [seahorse](./basics/transfer-sol/seahorse)

</details>
<details>
  <summary>Tokens</summary>

#### create-token

[Create a token on Solana with a token symbol and icon.](./tokens/create-token/README.md)

[anchor](./tokens/create-token/anchor), [native](./tokens/create-token/native)

#### escrow

Allow two users to swap digital assets with each other, each getting 100% of what the other has offered due to the power of decentralization!

[anchor](./tokens/escrow/anchor)

#### nft-minter

[Mint an NFT from inside your own onchain program using the Token and Metaplex Token Metadata programs.](./tokens/nft-minter/README.md) Reminder: you don't need your own program just to mint an NFT, see the note at the top of this README.

[anchor](./tokens/nft-minter/anchor), [native](./tokens/nft-minter/native)

#### pda-mint-authority

[Mint a Token from inside your own onchain program using the Token program.](./tokens/pda-mint-authority/README.md) Reminder: you don't need your own program just to mint an NFT, see the note at the top of this README.

[anchor](./tokens/pda-mint-authority/anchor), [native](./tokens/pda-mint-authority/native)

#### spl-token-minter

[Mint a Token from inside your own onchain program using the Token program.](./tokens/spl-token-minter/README.md) Reminder: you don't need your own program just to mint an NFT, see the note at the top of this README.

[anchor](./tokens/spl-token-minter/anchor), [native](./tokens/spl-token-minter/native)

#### token-swap

[Create liquidity pools to allow trading of new digital assets and allows users that provide liquidity to be rewarded by creating an Automated Market Maker.](./tokens/token-swap/README.md)

[anchor](./tokens/token-swap/anchor)

#### transfer-tokens

[Transfer tokens between accounts](./tokens/transfer-tokens/README.md)

[anchor](./tokens/transfer-tokens/anchor), [native](./tokens/transfer-tokens/native), [seahorse](./tokens/transfer-tokens/seahorse)

</details>

<details>

  <summary>Token Extensions</summary>

#### basics

Create token mints, mint tokens, and transferr tokens using Token Extensions.

[anchor](./tokens/token-2022/basics/anchor)

#### cpi-guard

Enable CPI guard to prevents certain token action from occurring within CPI (Cross-Program Invocation).

[anchor](./tokens/token-2022/cpi-guard/anchor)

#### default-account-state

Create new token accounts that are frozen by default.

[anchor](./tokens/token-2022/default-account-state/anchor), [native](./tokens/token-2022/default-account-state/native)

#### group

Create tokens that belong to larger groups of tokens using the Group Pointer extension.

[anchor](./tokens/token-2022/group/anchor)

#### immutable-owner

Create tokens whose owning program cannot be changed.

[anchor](./tokens/token-2022/immutable-owner/anchor)

#### interest-bearing

Create tokens that show an 'interest' calculation.

[anchor](./tokens/token-2022/interest-bearing/anchor)

#### memo-transfer

Create tokens where transfers must have a memo describing the transaction attached.

[anchor](./tokens/token-2022/memo-transfer/anchor)

#### metadata

Create tokens that store their onchain metadata inside the token mint, without needing to use or pay for additional programs.

[anchor](./tokens/token-2022/metadata/anchor)

#### mint-close-authority

Allow a designated account to close a Mint.

[anchor](./tokens/token-2022/mint-close-authority/anchor), [native](./tokens/token-2022/mint-close-authority/native)

#### multiple-extensions

Use multiple Token Extensions at once.

[native](./tokens/token-2022/multiple-extensions/native)

#### non-transferable

Create tokens that cannot be transferred.

[anchor](./tokens/token-2022/non-transferable/anchor), [native](./tokens/token-2022/non-transferable/native)

#### permanent-delegate

Create tokens that remain under the control of an account, even when transferred elsewhere.

[anchor](./tokens/token-2022/permanent-delegate/anchor)

#### transfer-fee

Create tokens

[anchor](./tokens/token-2022/transfer-fee/anchor), [native](./tokens/token-2022/transfer-fee/native)

</details>
<details>

<summary>Compression</summary>

#### cnft-burn

Burn compressed NFTs.

[anchor](./compression/cnft-burn/anchor)

#### cnft-vault

Store Metaplex compressed NFTs inside a PDA.

[anchor](./compression/cnft-vault/anchor)

#### cutils

Work with Metaplex compressed NFTs.

[anchor](./compression/cutils/anchor)

</details>

<details>

<summary>Oracles</summary>

#### pyth

Use a data source for offchain data (called an Oracle) to perform activities onchain.

[anchor](./oracles/pyth/anchor), [seahorse](./oracles/pyth/seahorse)

</details>

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
