# cnft-burn

This repository contains the cnft-burn program, a Solana Anchor program that allows you to burn compressed NFTs (cNFTs) in your collection. The program interacts with the Metaplex Bubblegum program through CPI to burn cNFTs.

## Components

- programs: Contains the anchor program
- tests: Contains the tests for the anchor program

## Deployment

The program is deployed on devnet at `FbeHkUEevbhKmdk5FE5orcTaJkCYn5drwZoZXaxQXXNn`. You can deploy it yourself by changing the respective values in lib.rs and Anchor.toml.

## How to run

1. Configure RPC path in utils/readAPI.ts. Personal preference: Helius RPCs.
2. run `anchor build` at the root of the project i.e cnft-burn in this case.
3. run `anchor deploy` to deploy and test the program on your own cluster.
4. to run the tests you need to have the tree address and the assetId of the cNFT you want to burn. You need to make sure the signer of the instruction is also the owner of that cNFT. if you don't have the tree and assetId you can create the tree and mint the cNFT using [compressed-nfts](https://github.com/solana-developers/compressed-nfts) and use the scripts `createAndMint.ts` to create the tree and get the tree address and `fetchNFTsByCollection.ts` to get the assetId based on the collection mint address which will be provided in the createAndMint script.
5. run `anchor test` to run the tests.

## Acknowledgements

This Example program would not have been possible without the work of:

- [Metaplex](https://github.com/metaplex-foundation/) for providing the Bubblegum program with ix builders.
- [@nickfrosty](https://twitter.com/nickfrosty) for providing the same code for fetching and creating cNFTs.
