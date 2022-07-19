# Minting an SPL Token to a Wallet

This example demonstrates how to mint an SPl Token to a Solana users's wallet.

### :key: Keys:

- The person requesting the mint must have an **associated token account** for that mint. We create this token account in the program!
- Steps:
    1. Create an associated token account for the Mint.
    2. Initialize that associated token account.
    3. Mint some amount of the Mint to the new token account.