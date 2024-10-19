# Transfer Hook Transfer Switch

This Solana program implements a transfer hook that can be toggled on or off for 
any token. It allows you to control whether transfers are allowed for a given wallet.

## Features

- **Initialize Transfer Control**: Set up a wallet with a default transfer status for a specified token mint.
- **Toggle Transfer On/Off**: Enable or disable token transfers for specific token mints.
- **Transfer Hook**: Enforces a transfer hook that checks the transfer status before allowing any token transfers.

## Prerequisites

- Rust and Cargo (for building the Solana program)
- Solana CLI (for interacting with Solana network)
- Anchor Framework (for Solana smart contract development)
- `pnpm` (for managing JavaScript dependencies)
- `solana-bankrun` for testing

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/solana-developers/program-examples.git
   cd program-examples/tokens/transfer-hook-transfer-switch/anchor
   ```

2. Install dependencies:
   ```
   pnpm install
   ```

## Building the Program

To build the program, run:

```
anchor build
```

## Testing

To run the test suite:

```
anchor test
```

## Program Structure

The program consists of three main instructions:

1. `initialize`: Creates and initializes the transfer switch account.
2. `toggle_transfer`: Toggles the transfer switch on or off.
3. `transfer_hook`: Checks if transfers are allowed before a token transfer.

## Usage

### Initializing the Transfer Switch

```typescript
await program.methods
  .initialize()
  .accounts({
    walletState: walletStatePda,
    owner: wallet.publicKey,
    tokenMint: tokenMintPublicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();
```

### Toggling the Transfer Switch

```typescript
await program.methods
  .toggleTransferSwitch(tokenMintPublicKey)
  .accounts({
    walletState: walletStatePda,
    owner: wallet.publicKey,
  })
  .rpc();
```

### Using the Transfer Hook

```typescript
await program.methods
  .transferTokens(new anchor.BN(amount))
  .accounts({
    from: fromTokenAccount,
    to: toTokenAccount,
    senderState: senderWalletStatePda,
    owner: ownerPublicKey,
    authority: transferAuthority,
    tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
  })
  .rpc();
```

## Account Structure

The `TransferSwitch` account has the following structure:

```rust
pub struct WalletState {
    pub owner: Pubkey, // The owner of the wallet
    pub transfer_status: Vec<(Pubkey, bool)>, // List of token mints and their transfer status
}
```

## Error Handling

The program defines a custom error:

```rust
#[error_code]
pub enum ErrorCode {
    #[msg("Transfers are disabled for this wallet.")]
    TransfersDisabled, // Triggered when transfers are disabled
    #[msg("Token not initialized for transfer management.")]
    TokenNotInitialized, // Triggered when a token mint has not been registered
}

```

This error is thrown when a transfer is attempted while the transfer switch is disabled.

## Security Considerations

- The `authority` of the `TransferSwitch` account has the power to toggle transfers on and off. Ensure this authority is properly managed and secured.
- This program does not implement any access control on who can transfer tokens. It only provides a global on/off switch for all transfers.
