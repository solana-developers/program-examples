# Solana Counter Program

A simple Solana program demonstrating the basics of on-chain state management, originally written in TypeScript and transpiled to Anchor using Poseidon. This program allows users to create and increment counters, with each user maintaining their own counter state.

## Development Approach

This program was initially developed in TypeScript for a more familiar development experience, then transpiled to Anchor using Poseidon. This approach makes it easier for TypeScript developers to get started with Solana development by:

- Writing programs in familiar TypeScript syntax
- Leveraging TypeScript's type safety and IDE support
- Automatically converting to Anchor's Rust-based framework
- Reducing the initial learning curve for Solana development

## Features

- Create a personal counter associated with your wallet
- Increment your counter
- Separate counter states for different users
- Program Derived Address (PDA) based account management

## Technical Details

### Program ID

```
3dhKkikKk112wEVdNr69Q2eEXSwU3MivfTNxauQsTjJP
```

### Account Structure

The program uses a `CounterAccount` to store the state with the following fields:

- `count: u64` - The current counter value
- `bump: u8` - PDA bump seed for account derivation

### Instructions

1. `initialize`
   - Creates a new counter account for the caller
   - Initializes count to 0
   - Stores the PDA bump

2. `increment`
   - Increments the counter by 1
   - Requires authority signature

## Development Setup

### Prerequisites

- Node.js and npm/yarn/pnpm
- Poseidon CLI (for TypeScript to Anchor transpilation)
- Rust and Cargo (for the transpiled Anchor program)
- Solana CLI tools
- Anchor CLI

### Building

1. Clone the repository

```bash
git clone https://github.com/solana-developers/program-examples.git
cd program-examples
```

2. Install dependencies

```bash
pnpm install
```

3. Transpile TypeScript to Anchor
```bash
poseidon compile --input "/ts-programs/src/counter.ts'" --output "/programs/counter/src/lib.rs"
```

4. Build the transpiled Anchor program
```bash
anchor build
```

### Testing

The program uses `anchor-bankrun` for testing, which provides a lightweight environment for running Solana program tests. Tests can be written in TypeScript and will work with both the original TypeScript program and the transpiled Anchor version.

Run the tests with:

```bash
anchor test
```

## Test Coverage

The test suite includes:

- Counter initialization verification
- Counter increment functionality
- Separate state management for different users

## Program Architecture

### Account Seeds

Counter accounts are derived using:

- Prefix: "counter"
- Authority public key

### Security Considerations

- Each counter is associated with a unique authority
- All modifications require authority signature
- PDA-based account derivation ensures account ownership

## Example Usage

```typescript
// Create a new counter
const { counter } = await program.methods
  .initialize()
  .accounts({
    authority: wallet.publicKey,
    counter: counterPDA,
    systemProgram: SystemProgram.programId,
  })
  .rpc();

// Increment the counter
await program.methods
  .increment()
  .accounts({
    authority: wallet.publicKey,
    counter: counterPDA,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

# Counter Program Tests Documentation

This document describes the test suite for the Solana counter program. The tests are written in TypeScript and work with both the original TypeScript program and the transpiled Anchor version using bankrun for testing.

## Test Setup

The test suite uses the following key components:

- `BankrunProvider`: A test provider that simulates the Solana runtime environment
- `Program<Counter>`: The Anchor program interface for the Counter program
- `Keypair`: Represents the authority (user) who can initialize and increment counters

### Key Dependencies

```typescript
import { BankrunProvider, startAnchor } from "anchor-bankrun";
import { Keypair, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
```

## Test Environment Setup

Before each test, the suite:

1. Generates a new authority keypair
2. Initializes a fresh bankrun context with:
   - Empty programs array
   - Initial account state including 10 SOL for the authority
3. Creates new instances of the BankrunProvider and Program

## Helper Functions

### `createCounter`

```typescript
async function createCounter(authority: Keypair) -> { counter: PublicKey, tx: string }
```

Creates a new counter account for the given authority by:

1. Deriving the PDA (Program Derived Address) using "counter" and the authority's public key
2. Initializing the counter account through the program
3. Returns the counter's address and transaction signature

## Test Cases

### 1. Counter Initialization

Tests that a new counter can be initialized with a count of 0.

```typescript
it("can initialize counter");
```

### 2. Counter Increment

Verifies that the counter can be incremented by 1.

```typescript
it("can increment counter");
```

### 3. Multiple Authority Separation

Tests that different authorities maintain separate counter states:

```typescript
it("maintains separate counts for different authorities");
```

- Creates two different authority keypairs
- Initializes separate counters for each
- Increments only the first counter
- Verifies that:
  - First counter shows count of 1
  - Second counter remains at 0

## Testing Considerations

1. Each test starts with a fresh state due to the `beforeEach` setup
2. Sufficient SOL (10 LAMPORTS_PER_SOL) is allocated to each authority
3. Program Derived Addresses (PDAs) are used to create unique counter accounts for each authority
4. Proper signing is maintained for all transactions

## Usage

To run these tests:

1. Ensure you have all dependencies installed
2. Build the TypeScript program
3. Transpile to Anchor using Poseidon
4. Run the test command: `anchor test`