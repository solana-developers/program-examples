# Poseidon Framework Implementation for Solana Program Examples

## Summary

This PR adds **Poseidon framework** implementations for two basic Solana programs:

1. **transfer-sol** - Demonstrates SOL transfers using SystemProgram CPI
2. **account-data** - Demonstrates creating and storing structured data in program-owned accounts

## What is Poseidon?

Poseidon is a TypeScript-to-Anchor transpiler framework by Turbin3 that enables developers to write Solana on-chain programs using TypeScript. The framework:
- Transpiles TypeScript code to Anchor/Rust
- Provides TypeScript-native syntax for Solana program development
- Lowers the barrier to entry for web developers new to Solana
- Generates production-ready Anchor code

**Note**: Poseidon is currently recommended for learning and testnet experimentation, not production mainnet applications.

## Implementation Details

### Directory Structure

Each implementation follows the established pattern with an additional TypeScript source directory:

```
basics/{program-name}/poseidon/
├── typescript/
│   └── {program_name}.ts (TypeScript source)
├── program/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs (transpiled Anchor code)
├── tests/
│   └── test.ts
├── Anchor.toml
├── package.json
└── tsconfig.json
```

### Key Features

**transfer-sol/poseidon:**
- TypeScript source demonstrates SystemProgram.transfer usage
- Transpiled to Anchor code with proper CPI handling
- Uses anchor-bankrun for testing
- Follows Anchor 1.0.0 patterns

**account-data/poseidon:**
- Creates program-owned accounts with structured data (name, house_number, street, city)
- Uses PDA derivation with seeds
- Demonstrates Anchor's InitSpace derive macro
- String fields with max_len constraints

### Testing

Both implementations include:
- TypeScript integration tests using anchor-bankrun
- Jest test framework
- Full test coverage matching existing framework implementations

### Build Commands

```bash
# Build program
pnpm build

# Run tests
pnpm build-and-test

# Deploy
pnpm deploy
```

## Changes Made

### Modified Files
- `Cargo.toml` - Added poseidon/program workspace members

### New Files
- `basics/transfer-sol/poseidon/` - Complete Poseidon implementation
- `basics/account-data/poseidon/` - Complete Poseidon implementation

## Bounty Information

This PR is submitted for the **Superteam Earn bounty**: "Create Solana Programs: Part 1"
- Bounty ID: 3a73f71c-5cc4-4b72-ae63-028651ed3655
- Reward: $200-500 USDC per program per framework
- Framework: Poseidon (new framework addition)
- Programs: transfer-sol ($200), account-data ($300)

## Testing Checklist

- [x] Programs compile successfully with `cargo check`
- [x] TypeScript source files follow Poseidon patterns
- [x] Transpiled Anchor code uses anchor-lang 1.0.0
- [x] Tests use anchor-bankrun
- [x] Follows existing project structure
- [x] Package.json includes required scripts
- [x] Workspace Cargo.toml updated

## Related Documentation

- Poseidon Framework: https://github.com/Turbin3/poseidon
- Helius Blog: https://www.helius.dev/blog/build-solana-programs-with-typescript-poseidon
- Anchor Framework: https://www.anchor-lang.com
- Contributing Guidelines: CONTRIBUTING.md

---

**Author**: Claude Opus 4.6 (wangxiaofei860208-source)  
**Submitted**: 2026-04-17
