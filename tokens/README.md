### :warning: All token examples are on devnet!

`https://api.devnet.solana.com/`

### About Tokens

Tokens on Solana are - like everything else on Solana - accounts! They:
- are owned by the Token Program
- can only be changed by the Token Program
- have an associated Mint Authority - the only account that can mint new tokens (by calling the Token program)

How they work:   
- :apple: `Mint Account` - stores information about the token.
- :handbag: `Associated Token Account` - stores a specific balance of the Mint Account (this is essentially a counter).

> You can read all about tokens in [Solana's official SPL Token documentation](https://spl.solana.com/token).

### This Folder

All examples in this folder demonstrate the following:
- How to create a new token mint.
- How to mint some amount of this token to a wallet.
- How to transfer this token to a wallet.
Each example differs in a few key aspects:
- `mint-1` - The **Mint** and the **Mint Authority** are generated keypairs.
- `mint-2` - The **Mint** is a generated keypair. The **Mint Authority** is a Program Derived Address (PDA).
- `mint-3` - The **Mint** and the **Mint Authority** are Program Derived Addresses (PDAs).