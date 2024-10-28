# Counter Program Example

A minimal example demonstrating a counter program using Solana Turbine.

## Features
- Initialize counter account
- Increment counter value
- Authority-based access control

## Quick Start
```bash
# Install dependencies
npm install

# Run tests
npm test
```

## Usage
```typescript
// Create program instance
const program = new CounterProgram();

// Initialize counter
await program.initialize(authority, counter);

// Increment counter
await program.increment(authority, counter);
```

## Security Considerations
- Only the authority can increment the counter
- PDA derivation ensures account ownership
- Bump seed stored for efficient verification
