## SET TRANSFER DELEGATE PLUGIN

This example shows you how to set the mpl-core [Transfer Delegate](https://developers.metaplex.com/core/plugins/transfer-delegate) Plugin.

Similar to how Transfer Delegate works with pNFTs, transfer delegate allows the owner/authority of an asset transfer authority over the asset.

Check out mpl-core plugin docs here - [developers.metaplex.com/core/plugins](https://developers.metaplex.com/core/plugins)

Check out Program code here - [github.com/metaplex-foundation/mpl-core](https://github.com/metaplex-foundation/mpl-core)

## Getting started

Here is how my environment looks like

```bash
solana-cli 1.18.9 # install by running $ solana-install 1.18.9
nchor-cli 0.29.0 # install by running $ avm install latest
```

To use this example,

1. Build the anchor project

```bash
anchor build
```

2. List the program keys

```bash
anchor keys list
```

3. Update the [Anchor.toml](./Anchor.toml) and [lib.rs](./programs/transfer-delegate/src/lib.rs) with new program address

4. Running the Tests

-   Update the [transfer-delegate](./tests/transfer-delegate.ts) file with correct `Asset` and `Delegate` address
-   Run the test

```bash
anchor test
```
