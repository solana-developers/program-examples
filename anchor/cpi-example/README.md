# Cross-Program Invocation (CPI) Example

This example demonstrates how to use **Cross-Program Invocations (CPI)** in Anchor programs. CPI allows one Solana program to call functions from another program, enabling powerful composability and code reuse.

## What is CPI?

Cross-Program Invocation (CPI) is a mechanism that allows a Solana program to invoke instructions from other programs. This is fundamental for building composable applications where programs can interact with each other seamlessly.

## What This Example Demonstrates

This example includes:

1. **CPI to Token Program**: CPI call to transfer tokens using the SPL Token program
2. **CPI to System Program**: CPI call to transfer SOL using the System program
3. **Multiple CPI Calls**: Single instruction that performs multiple CPI calls
4. **CPI with PDA Authority**: CPI call using a Program Derived Address as authority
5. **State Management**: How to track and manage state across CPI calls

## Program Structure

### CPI Example Program (`cpi_example`)
The main program that demonstrates various CPI patterns:
- `initialize`: Initialize the CPI example account
- `transfer_tokens_via_cpi`: Call token program's transfer function via CPI
- `transfer_sol_via_cpi`: Call system program's transfer function via CPI
- `multiple_cpi_calls`: Perform multiple CPI calls in a single instruction
- `transfer_with_pda_authority`: CPI call using a PDA as authority

## Key CPI Concepts Demonstrated

### 1. CPI Context Creation
```rust
let cpi_ctx = CpiContext::new(
    ctx.accounts.token_program.to_account_info(),
    Transfer {
        from: ctx.accounts.from_token_account.to_account_info(),
        to: ctx.accounts.to_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    },
);
```

### 2. CPI Call Execution
```rust
token::transfer(cpi_ctx, amount)?;
```

### 3. CPI to System Programs
```rust
anchor_lang::system_program::transfer(sol_cpi_ctx, amount)?;
```

### 4. CPI with PDA Authority
```rust
let cpi_ctx = CpiContext::new_with_signer(
    ctx.accounts.token_program.to_account_info(),
    Transfer { /* ... */ },
    signer_seeds,
);
```

### 5. Multiple CPI Calls
```rust
// First CPI call
token::transfer(token_cpi_ctx, token_amount)?;
// Second CPI call
anchor_lang::system_program::transfer(sol_cpi_ctx, sol_amount)?;
```

## Running the Example

### Prerequisites
- [Anchor](https://www.anchor-lang.com/) installed
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) installed
- Node.js and npm/yarn installed

### Build and Test
```bash
# Build the programs
anchor build

# Run tests
anchor test
```

### Manual Testing
```bash
# Deploy to localnet
anchor deploy

# Run specific test
anchor test --skip-local-validator
```

## Test Scenarios

The test suite covers:

1. **Initialization**: Setting up the CPI example program and accounts
2. **Token Setup**: Creating a mint and token accounts for testing
3. **Token CPI**: Token transfer via CPI
4. **SOL CPI**: SOL transfer via CPI
5. **Multiple CPI**: Single instruction with multiple CPI calls
6. **PDA Authority CPI**: CPI call using a PDA as authority
7. **State Verification**: Ensuring all state changes are correct

## Key Learning Points

### 1. Account Management
- CPI requires proper account setup in the calling program
- All accounts needed by the target program must be provided
- Account ownership and permissions must be correctly configured

### 2. Error Handling
- CPI calls can fail, so proper error handling is essential
- Use `?` operator or `Result` handling for CPI calls
- Consider rollback scenarios for multiple CPI calls

### 3. State Consistency
- Track state changes across CPI calls
- Ensure atomicity when performing multiple operations
- Consider what happens if a CPI call fails mid-execution

### 4. Security Considerations
- Validate all inputs before CPI calls
- Ensure proper authority checks
- Be aware of reentrancy possibilities

## Common Use Cases for CPI

1. **Token Operations**: Transferring, minting, or burning tokens
2. **Account Creation**: Creating accounts using system program
3. **Program Composition**: Building complex functionality by combining simpler programs
4. **State Management**: Updating state across multiple programs
5. **DeFi Operations**: Interacting with lending, swapping, or staking programs

## Best Practices

1. **Always validate inputs** before making CPI calls
2. **Use proper error handling** and consider rollback scenarios
3. **Minimize the number of CPI calls** per instruction when possible
4. **Document CPI dependencies** clearly in your program
5. **Test thoroughly** with various failure scenarios
6. **Consider gas costs** of multiple CPI calls

## Further Reading

- [Anchor CPI Documentation](https://www.anchor-lang.com/docs/cross-program-invocations)
- [Solana Program Library](https://github.com/solana-labs/solana-program-library)
- [SPL Token Program](https://spl.solana.com/token)
- [Solana Cross-Program Invocation Guide](https://docs.solana.com/developing/programming-model/calling-between-programs)

## Troubleshooting

### Common Issues

1. **Account Not Found**: Ensure all required accounts are provided in the CPI context
2. **Insufficient Funds**: Check that accounts have enough lamports for rent
3. **Wrong Program ID**: Verify the program ID in the CPI context matches the target program
4. **Authority Mismatch**: Ensure the signer has proper authority for the operation

### Debug Tips

1. Use `msg!` macros to log values during CPI calls
2. Check account states before and after CPI calls
3. Verify program IDs and account relationships
4. Use Solana Explorer to inspect transaction details
