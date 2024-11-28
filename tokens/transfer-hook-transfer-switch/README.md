# Transfer Hook
The Transfer Hook extension and Interface allow Mint Accounts to execute custom logic on every token transfer. This enables use cases like enforcing NFT royalties, managing wallet blacklists/whitelists, applying custom fees, and tracking transfer stats. 

see: https://solana.com/ja/developers/guides/token-extensions/transfer-hook

This example demonstrates how to implement a Transfer Hook program that enforces a transfer switch on a token. The transfer switch is a boolean flag that allows or disallows token transfers. The Transfer Hook program checks the switch before executing the token transfer.

