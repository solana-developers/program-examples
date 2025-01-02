# ProcessingInstructions

See the [Processing instructions's README](../README.md) for more information. In our case, we cannot use Borsh for serialization, as we're constrained by the `Steel` framework dependency on PODs (Plain Old Data).

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
