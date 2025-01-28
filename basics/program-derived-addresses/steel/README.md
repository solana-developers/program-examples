# Program Derived Addresses

This program demonstrates how to derive addresses. It will use a PDA to store a counter of visits and increment it.

## Building

```sh
cargo build-sbf

```
## Tests

This project includes both:
- Rust tests: [`program/tests`](/program/tests) directory.
- Node.js tests using [Bankrun](https://kevinheavey.github.io/solana-bankrun/): [`tests`](/tests) directory.

```sh
# rust tests
cargo test-sbf 

# node tests
pnpm build-and-test # this will also build the program
#or 
pnpm test # if you have already built the program
```
