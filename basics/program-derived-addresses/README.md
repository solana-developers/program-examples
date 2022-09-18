# Program Derived Addresses (PDA)

A program derived addresses are accounts that are controlled by programs, unlike normal account that are
controlled by the private key corresponding to the public key that serves as the address of the account.

With normal account, a transaction that changes the state of the account needs to be signed by the private key
corresponding to the account. With PDA's, there is no private key corresponding to the account, hence any
transaction that changes the state of the account needs to invoked by the Solana program that controls the PDA.

You can think of PDA as database which a Solana program controls.

### Links:
- [Solana Docs - Program Derived Addresses](https://docs.solana.com/developing/programming-model/calling-between-programs#program-derived-addresses)