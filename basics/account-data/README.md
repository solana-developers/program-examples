# Account Data

:package: We're going to store data in a Solana account. :memo:

Solana accounts are used to store persistent state. This means that the data stays on the blockchain until the account is closed.

In this example, we demonstrate how to:
1.  **Define a data structure**: We create an `AddressInfo` struct to store a name, house number, street, and city.
2.  **Calculate required space**: We determine how much memory (bytes) the account needs to store our structure.
3.  **Initialize the account**: We allocate the space and write the initial data.

### Frameworks covered:
- **Anchor**: Uses `#[account]` and `#[derive(InitSpace)]` to automate space calculation.
- **Native**: Manually calculates space and uses `Borsh` for serialization.
- **Pinocchio**: A lightweight approach to account data management.

Each implementation shows how to handle fixed-size data and strings with maximum lengths.

### Links:
- [Solana Docs - Accounts](https://docs.solana.com/developing/programming-model/accounts)
- [Anchor Docs - Space Reference](https://www.anchor-lang.com/docs/space)
- [Borsh Specification](https://borsh.io/)
