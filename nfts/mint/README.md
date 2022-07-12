# Create a New NFT Mint

:notebook_with_decorative_cover: Note: This example is built off of [Mint Token](../../tokens/mint/README.md) and [Mint Token To](../../tokens/mint-to/README.md). If you get stuck, check out those examples.   

___

An NFT is obviously just a token on Solana! So, the process is the same for creating an NFT. There's just a few additional steps:
- Decimals are set to 0
- Minting must be disabled after one token is minted (ie. cap the supply at 1).