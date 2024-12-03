# Transfer Hook Transfer Switch

A Solana program that implements token transfer controls using a hook pattern, allowing wallets to be frozen and unfrozen for tokens.

## Prerequisites

- Rust and Cargo
- Solana CLI tools
- Anchor CLI
- Node.js and Yarn

## Setup

1. Clone the repository

```bash
   git clone <repository-url>
   cd <project-directory>
```

2. Install dependencies:

```bash
   pnpm install
```

3. Build the Anchor project

```bash
   anchor build
```

4. Sync the program ID:

```bash
  anchor keys sync
```

## Running Tests

```bash
anchor test
```

## Notes

- Ensure your Solana validator is running locally before running tests.
- If you encounter any issues, make sure your Anchor.toml and Cargo.toml files are correctly configured for your project.
