### :warning: All token examples are on devnet!

`https://api.devnet.solana.com/`

### About Tokens

Tokens on Solana are - like everything else on Solana - accounts! They:
- are owned by the Token Program
- can only be changed by the Token Program
- have an associated Mint Authority - the only account that can mint new tokens (by calling the Token program)

How they work:   
:apple: `Mint Account` - stores information about the token.
:handbag: `Associated Token Account` - stores a specific balance of the Mint Account (this is essentially a counter).

> You can read all about tokens in [Solana's official SPL Token documentation](https://spl.solana.com/token).