# Contribution Guidelines

Thank you for considering contributing to the Solana Program Examples repository. We greatly appreciate your interest and efforts in helping us improve and expand this valuable resource for the Solana developer community.

We believe that a welcoming and inclusive environment fosters collaboration and encourages participation from developers of all backgrounds and skill levels.

To ensure a smooth and effective contribution process, please take a moment to review and follow the guidelines outlined below.

## How to Contribute

We welcome contributions in the form of code, documentation, bug reports, feature requests, and other forms of feedback. Here are some ways you can contribute:

- **Code Contributions:** You can contribute code examples in Rust, Python, or Solidity that demonstrate various Solana program functionalities. You can also contribute improvements to existing examples, such as bug fixes, optimizations, or additional features.

- **Bug Reports, Ideas or Feedback:** If you encounter any issues or have ideas for new examples, please submit a bug report or feature request. Your feedback is valuable and helps us improve the quality and relevance of the examples.

## General coding and writing guidelines

Please follow the [Contributing and Style Guide from the Developer Content Repo](https://github.com/solana-foundation/developer-content/blob/main/CONTRIBUTING.md).

Specifically for code in this repo:

1. Use pnpm as the default package manager for the project. You can [install pnpm by following the instructions](https://pnpm.io/installation). Commit `pnpm-lock.yaml` to the repository.

2. Solana Programs written for Anchor framework  should be in directory (`anchor`)[https://www.anchor-lang.com], Solana Native in (`native`)[https://solana.com/developers/guides/getstarted/intro-to-native-rust], Steel Framework in (`steel`)[https://github.com/regolith-labs/steel], TypeScript in (`poseidon`)[https://github.com/Turbin3/poseidon], respectively.
  - Project path structure: `/program-examples/category/example-name/<framework_name>`
    - Project path structure example for anchor: `/program-examples/category/example-name/anchor`

3. Tests for Solana native programs, steel framework programs, and Anchor should be written with [solana-bankrun](https://kevinheavey.github.io/solana-bankrun)

4. Steel framework programs must be organized as a Cargo workspace with separate projects for API and program:
   - Project path structure: `/program-examples/category/example-name/steel`
   - Initialise project using `steel new <name>`
   - Must be a Cargo workspace with two separate projects:
     - `api`: Contains API-related code
     - `program`: Contains the program implementation
     - Steel projects should NOT be added in the root [`Cargo.toml` file](https://github.com/solana-developers/program-examples/blob/main/Cargo.toml)

   This structure ensures proper organization and separation of concerns.

5. For Steel framework programs:
   - Steel CLI is the recommended way to build and test programs:
     ```bash
     # Install Steel CLI (one-time setup)
     cargo install steel-cli

     # Create a new Steel project
     steel new <name>

     # Build the program
     steel build

     # Run tests
     steel test
     ```
   - Alternatively, you can use package.json scripts if you need custom build/test configurations as Solana native one described below.

6. For Solana native programs ensure adding these mandatory pnpm run scripts to your `package.json` file for successful CI/CD builds:

```json
"scripts": {
  "test": "pnpm ts-mocha -p ./tests/tsconfig.test.json -t 1000000 ./tests/realloc.test.ts",
  "build-and-test": "cargo build-sbf --manifest-path=./program/Cargo.toml --sbf-out-dir=./tests/fixtures && pnpm test",
  "build": "cargo build-sbf --manifest-path=./program/Cargo.toml --sbf-out-dir=./program/target/so",
  "deploy": "solana program deploy ./program/target/so/program.so"
},
```

Alternatively, You can add `steel test` and `steel build` as commands according to your project.

"scripts": {
  "test": "steel test",
  "build-and-test": "steel build && steel test",
  "build": "steel build",
  "deploy": "solana program deploy ./program/target/so/program.so"
},

7. Test command for Anchor should execute `pnpm test` instead of `yarn run test` for anchor programs. Replace `yarn` with `pnpm` in `[script]` table inside [Anchor.toml file.](https://www.anchor-lang.com/docs/manifest#scripts-required-for-testing)

```
[scripts]
test = "pnpm ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
```

8. TypeScript, JavaScript and JSON files are formatted and linted using
   [Biome](https://biomejs.dev/). Execute the following command to format and lint your code at the root of this project before submitting a pull request:

```bash
pnpm fix
```

9. Some projects can be ignored from the building and testing process by adding the project name to the `.gitignore` file.
When removing or updating an example, please ensure that the example is removed from the `.gitignore` file
and there's a change in that example's directory.

## Code of Conduct

We are committed to providing a friendly, safe, and welcoming environment for all contributors, regardless of their background, experience level, or personal characteristics. As a contributor, you are expected to:

Be respectful and inclusive in your interactions with others.
Refrain from engaging in any form of harassment, discrimination, or offensive behavior. Be open to constructive feedback and be willing to learn from others.
Help create a positive and supportive community where everyone feels valued and respected.

If you encounter any behavior that violates our code of conduct, please report it to the project maintainers immediately.
