## Mint Asset

This example shows you how to mint a core asset using [`Metaplex Core`](https://developers.metaplex.com/core) program.

As will be demonstrated by the example, one of the main advantages for developer is the reduced number of accounts needed to mint an asset.

Check out core docs here - [developers.metaplex.com/core](https://developers.metaplex.com/core)

Check out Program code here - [github.com/metaplex-foundation/mpl-core](https://github.com/metaplex-foundation/mpl-core)

## Getting started

Here is how my environment looks like

```bash
solana-cli 1.18.9 # install by running $ solana-install 1.18.9
nchor-cli 0.29.0 # install by running $ avm install latest
```

1. Initialize Anchor Project

```bash
anchor init mint-asset
```

2. Install crates

```bash
cargo add mpl-core
```

## Building

```bash
anchor build
```

## Testing

```bash
anchor test
```
