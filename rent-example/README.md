# Rent Example with Steel and Poseidon

This example demonstrates how rent works on Solana using the Steel and Poseidon frameworks. In this example, we’ll explore how accounts can be made rent-exempt by holding enough balance and how to check if an account is rent-exempt.
Overview

In Solana, storage incurs a small rent fee, which is required to keep accounts active on the blockchain. However, if an account holds enough balance to cover rent for approximately two years, it becomes rent-exempt, meaning no rent is charged.

This example provides a simple demonstration of:

    Checking if an account has enough funds to be rent-exempt.
    Making sure accounts stay rent-exempt by providing adequate funding.

## Dependencies

To run this example, make sure you have the following dependencies installed:

    Rust and Cargo: Install Rust
    Anchor: For Solana smart contract development.

    bash

cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest
avm use latest

Solana CLI: For interacting with the Solana blockchain.

bash

    sh -c "$(curl -sSfL https://release.solana.com/v1.10.29/install)"

    Node.js and NPM: Required for running JavaScript or TypeScript tests.
        Install Node.js and npm from nodejs.org.

## Setup Instructions

    Clone the Repository
    Clone your fork of the Solana program-examples repository:

    bash

git clone https://github.com/Luckingz/program-examples.git
cd program-examples/basics/rent_example

Build the Program Run the following command to build the program:

bash

anchor build

Deploy the Program Deploy the program to your local Solana cluster (or to devnet if needed).

bash

    anchor deploy

## Running the Example

To run this example, use the following steps:

    Check Account Rent Exemption Status The example allows you to check if an account has sufficient balance to be rent-exempt.

    bash

    anchor test

    Expected Output
    The tests will validate:
        Whether an account is rent-exempt.
        If the account requires additional balance to achieve rent exemption.

    If the test passes, it confirms that the account is successfully validated for rent exemption.

## Testing

We have provided tests written in TypeScript to verify that the example works as expected. The tests:

    Check if an account meets rent exemption requirements.
    Confirm that accounts behave as expected under Solana's rent model.

To run the tests, use:

bash

anchor test

Notes

    Make sure you have a funded wallet on your local network to cover the transaction fees.
    Customize any rent values or exemption thresholds in tests/rent_example.ts based on your needs.
