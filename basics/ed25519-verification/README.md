# Ed25519 Signature Verification for Custodied Funds

This example demonstrates how to implement Ed25519 signature verification to manage custodied funds on Solana. The program verifies Ed25519 signatures before allowing transfers from custodial accounts.

## Overview

The example shows how to:
- Verify Ed25519 signatures using Solana's native Ed25519 program
- Transfer funds from custodial accounts after signature verification
- Implement secure authorization checks
- Handle signature verification errors

## Quick Start

The example is implemented in multiple frameworks:

### Native
```bash
cd native
pnpm install
pnpm build-and-test
```

### Anchor
```bash
cd anchor
pnpm install
anchor build
pnpm test
```

### Steel
```bash
cd steel
pnpm install
steel build
steel test
```

### Poseidon (TypeScript)
```bash
cd poseidon
pnpm install
pnpm test
```

## Program Structure

The program consists of the following key components:

1. **Signature Verification**: Uses Solana's Ed25519 program to verify signatures
2. **Fund Transfer**: Handles secure transfer of funds after verification
3. **Account Validation**: Ensures proper account permissions and ownership

### Account Structure
- Custodial Account: Holds the funds
- Recipient: Account receiving the funds
- Signer: Account authorized to initiate transfers
- Ed25519 Program: Solana's native signature verification program

### Instruction Data
```rust
pub struct TransferInstruction {
    signature: [u8; 64],    // Ed25519 signature
    public_key: [u8; 32],   // Signer's public key
    amount: u64,            // Transfer amount in lamports
    message: Vec<u8>,       // Message that was signed
}
```

## Usage

### Creating a Transfer

```typescript
// Create and sign the transfer message
const message = Buffer.from(`Transfer ${amount} lamports to ${recipient.toBase58()}`);
const signature = await sign(message, signerKeypair.secretKey.slice(0, 32));

// Create the instruction
const instruction = new TransactionInstruction({
  keys: [
    { pubkey: custodialAccount, isSigner: false, isWritable: true },
    { pubkey: recipient, isSigner: false, isWritable: true },
    { pubkey: signerKeypair.publicKey, isSigner: true, isWritable: false },
    { pubkey: ed25519ProgramId, isSigner: false, isWritable: false },
  ],
  programId,
  data: Buffer.concat([signature, publicKey, amount, message]),
});
```

### Security Considerations

1. **Signature Verification**: Always verify signatures before transferring funds
2. **Account Validation**: Check account ownership and permissions
3. **Error Handling**: Properly handle all error cases
4. **Amount Validation**: Verify sufficient funds before transfer

## Testing

Each implementation includes comprehensive tests demonstrating:
- Successful signature verification and transfer
- Handling of invalid signatures
- Error cases for insufficient funds
- Account permission checks

## Framework-Specific Details

### Native Implementation
- Direct Solana program implementation
- Manual account and instruction handling
- Bankrun tests for verification

### Anchor Implementation
- Uses Anchor's account validation
- Structured instruction handling
- Type-safe client interface

### Steel Implementation
- Separated API and program logic
- Steel-specific optimizations
- Integrated testing tools

### Poseidon Implementation
- TypeScript client implementation
- Modern Solana practices
- Versioned transaction support