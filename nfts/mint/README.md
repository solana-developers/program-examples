# Create a New SPL Token Mint

This example demonstrates how to create an SPl Token on Solana with some metadata such as a token symbol and icon.

### :key: Keys:

- SPL Tokens by default have **9 decimals**, and **NFTs have 0 decimals**. "Decimals" here means the number of decimal; ie. a token with 3 decimals will be tracked in increments of 0.001.   
- You can use [Metaplex's Token Metadata Program](https://docs.metaplex.com/) to create metadata for your token.
- Steps:
    1. Create an account for the Mint.
    2. Initialize that account as a Mint Account.
    3. Create a metadata account associated with that Mint Account.