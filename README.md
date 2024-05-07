# Program Examples

### :crab: Rust. :snake: Python. :ice_cube: Solidity. :link: All on-chain.

> **New:** Solidity examples from Solang!

## Navigating this Repo

:file_folder: Each example contains four folders:
- `native` - Written using Solana's native Rust crates and vanilla Rust.
- `anchor` - Written using Anchor's `anchor_lang` Rust crate and the associated Anchor framework to build & deploy.
- `seahorse` - Written using the Python framework Seahorse, which converts your Python code to Anchor Rust.
- `solang` - Written using the Solang compiler, which allows developers to run Solidity natively on Solana.

:wrench: How to build & run:
- Make sure to have [pnpm](https://pnpm.io/installation) and [Solana CLI and Anchor](https://solana.com/developers/guides/getstarted/setup-local-development) installed
- Before running anything in any folder make sure you pull in the dependencies with `pnpm install`.
- `native` - Use `cicd.sh` to build & deploy the program. Run `pnpm test` to test it.
- `anchor` - Use `anchor build && anchor deploy` to build & deploy the program. Run `anchor test` to test it.
- `seahorse` - Use `seahorse build && anchor deploy` to build & deploy the program. Run `anchor test` to test it.
- `solang` - Use `anchor build && anchor deploy` to build & deploy the program. Run `anchor test` to test it.

## Contributing

This project welcomes contributions and suggestions.
If you'd like to contribute, please fork the repository and use a feature branch. Pull requests are warmly welcome.
Read the [CONTRIBUTING.md](.github/CONTRIBUTING.md) file for more information.
Failing to follow the guidelines may result in your PR being closed.

## Examples We'd Love to See!

* Examples needed for Native:
    * Token2022
* Examples needed for Anchor:
    * Additional Accounts & Resolving Accounts
* Examples needed for Seahorse
    * Any existing example missing a `seahorse` folder
* Examples needed for Solidity
    * Any existing example missing a `solang` folder
* New examples needed for Solidity, Anchor, Native & Seahorse:
    * Token lending
    * Token swapping
    * Escrow
    * Staking
    * Wrapped tokens
    * Pyth
    * Clockwork
    * VRF
    * Any oracle
    * Merkle trees (compression)

---

## If You're New To Solana Please Read
Most system-level operations on Solana involve already-existing Solana programs.

For example, to create a **system account** you use the **system program** and to create a **token mint** you use the **token program**.

So, you'll notice that these operations are in fact conducting what's called a **cross-program invocation** - which is a fancy way of saying it calls other Solana programs to do business. You can see this in action whenever you see `invoke` or `invoke_signed` in the `native` examples, or `CpiContext` in the `anchor` examples.

Deciding when to use cross-program invocation instead of invoking the programs directly from the client is completely up to you as the builder. It depends on how your application is designed.
- Maybe you want to add some checks - such as minimum balance required, allowed ownership, etc.
- Maybe you want to assert that an account has a certain data type.
- Perhaps you want to send only one instruction from your client for a handful of sequential operations.
- The list goes on.
Regardless of what you may want to add on top of existing Solana programs, the number one use case for writing your own program is for using accounts with a **Program Derived Address (PDA)**. Crack open the `pdas` folder to see why.
