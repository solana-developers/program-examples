# Program Derived Addresses (PDA)

Program derived addresses allow programmatically generated signatures to be used when [calling between programs](https://docs.solana.com/developing/programming-model/calling-between-programs#cross-program-invocations).

Using a program derived address, a program may be given the authority over an account and later transfer that 
authority to another. This is possible because the program can act as the signer in the transaction that gives 
authority. You can think of PDA as database which a Solana program controls.

### Links:
- [Solana Docs - Program Derived Addresses](https://docs.solana.com/developing/programming-model/calling-between-programs#program-derived-addresses)