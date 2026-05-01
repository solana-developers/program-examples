# Temporarily relocated Rust tests

`test.rs` was moved here from `programs/abl-token/tests/test.rs` because it
cannot currently compile alongside anchor-lang 0.32.1.

## Why

The test uses **litesvm**, which pins `solana-account-info` to an exact 2.2.x
version. However, anchor-lang 0.32.1 requires `solana-account-info >=2.3.0`
for the `AccountInfo::resize()` method (renamed from `realloc` in 2.3.0).

This creates an unresolvable dependency conflict:

- **litesvm <=0.6** → pins `solana-account-info =2.2.1` (too old, no `resize()`)
- **litesvm 0.9+** → uses `solana-account-info 3.x` (type mismatch with anchor-lang's 2.x types)

No litesvm release currently targets the `solana-account-info 2.3+` range
that anchor-lang 0.32.1 needs.

## When to move it back

Move `test.rs` back to `programs/abl-token/tests/test.rs` and restore the
`[dev-dependencies]` in `Cargo.toml` when **either**:

1. **anchor-lang upgrades to solana 3.x** (likely 0.33+), so litesvm 0.9+ types match, or
2. **litesvm releases a version** targeting `solana-account-info >=2.3, <3`
