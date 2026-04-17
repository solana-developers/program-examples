# Steel Framework Implementation for Solana Program Examples

## Summary

This PR adds **Steel framework** implementations for two basic Solana programs:

1. **transfer-sol** - Demonstrates SOL transfers using both CPI and direct lamport manipulation
2. **account-data** - Demonstrates creating and storing structured data in program-owned accounts

## What is Steel?

Steel is a lightweight Solana program framework (v4.0.4) by Regolith Labs that provides:
- Minimal boilerplate with helper macros (`account!`, `instruction!`, `error!`, `event!`)
- Byte-based serialization using `bytemuck` (Pod/Zeroable traits)
- Account validation helpers (`is_signer()`, `is_writable()`, `has_owner()`, etc.)
- Type-safe instruction parsing with `parse_instruction()`
- Chainable validation methods for cleaner code

## Implementation Details

### Directory Structure

Each implementation follows the established pattern:

```
basics/{program-name}/steel/
├── program/
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs
│   └── tests/
│       └── test.rs
├── tests/
│   ├── test.ts
│   └── instruction.ts (transfer-sol only)
├── package.json
└── tsconfig.json
```

### Key Features

**transfer-sol/steel:**
- Implements two transfer methods: CPI via system program and direct lamport manipulation
- Uses Steel's `parse_instruction()` for type-safe instruction parsing
- Demonstrates Steel's account validation helpers
- Includes both Rust (litesvm) and TypeScript (solana-bankrun) tests

**account-data/steel:**
- Creates program-owned accounts with structured data
- Uses `bytemuck` for zero-copy serialization
- Demonstrates rent calculation and account creation via CPI
- Fixed-size byte arrays for string fields (32 bytes each)

### Testing

Both implementations include:
- Rust unit tests using `litesvm` (program/tests/test.rs)
- TypeScript integration tests using `solana-bankrun` (tests/test.ts)
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
- `Cargo.toml` - Added steel/program workspace members and steel dependency

### New Files
- `basics/transfer-sol/steel/` - Complete Steel implementation
- `basics/account-data/steel/` - Complete Steel implementation

## Bounty Information

This PR is submitted for the **Superteam Earn bounty**: "Create Solana Programs: Part 1"
- Bounty ID: 3a73f71c-5cc4-4b72-ae63-028651ed3655
- Reward: $200-500 USDC per program per framework
- Framework: Steel (new framework addition)
- Programs: transfer-sol ($200), account-data ($300)

## Testing Checklist

- [x] Programs compile successfully with `cargo check`
- [x] Rust tests pass with `cargo test`
- [x] TypeScript tests use solana-bankrun
- [x] Code formatted with Biome (`pnpm fix`)
- [x] Follows existing project structure
- [x] Package.json includes required scripts
- [x] Workspace Cargo.toml updated

## Related Documentation

- Steel Framework: https://github.com/regolith-labs/steel
- Steel Docs: https://docs.rs/steel/latest/steel/
- Contributing Guidelines: CONTRIBUTING.md

---

**Author**: Claude Opus 4.6 (wangxiaofei860208-source)  
**Submitted**: 2026-04-17
