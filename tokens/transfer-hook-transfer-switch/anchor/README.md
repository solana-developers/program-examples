# Transfer Hook Transfer Switch

This Solana program implements a transfer hook that can be toggled on or off for 
any token. It allows you to control whether transfers are allowed for a given wallet.

## Features

- Initialize a transfer switch
- Toggle transfers on/off
- Transfer hook that checks if transfers are allowed

## Prerequisites

- Rust and Cargo
- Solana CLI
- Anchor Framework

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/solana-developers/program-examples.git
   cd program-examples/tokens/transfer-hook-transfer-switch/anchor
   ```

2. Install dependencies:
   ```
   npm install
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
    transferSwitch: transferSwitchPda,
    payer: wallet.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();
```

### Toggling the Transfer Switch

```typescript
await program.methods
  .toggleTransfer(true) // or false to disable
  .accounts({
    transferSwitch: transferSwitchPda,
    authority: wallet.publicKey,
  })
  .rpc();
```

### Using the Transfer Hook

```typescript
await program.methods
  .transferHook(new anchor.BN(amount))
  .accounts({
    transferSwitch: transferSwitchPda,
    from: fromPublicKey,
    to: toPublicKey,
    tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
  })
  .rpc();
```

## Account Structure

The `TransferSwitch` account has the following structure:

```rust
pub struct TransferSwitch {
    pub is_enabled: bool,
    pub authority: Pubkey,
}
```

## Error Handling

The program defines a custom error:

```rust
#[error_code]
pub enum TransferSwitchError {
    #[msg("Transfers are currently disabled")]
    TransfersDisabled,
}
```

This error is thrown when a transfer is attempted while the transfer switch is disabled.

## Security Considerations

- The `authority` of the `TransferSwitch` account has the power to toggle transfers on and off. Ensure this authority is properly managed and secured.
- This program does not implement any access control on who can transfer tokens. It only provides a global on/off switch for all transfers.
