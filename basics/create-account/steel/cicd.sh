#!/bin/bash

# Buld and deploy this program with ease using a single command
# Run this script with "bash cicd.sh" or "./cicd.sh"
# Note: Try running "chmod +x cicd.sh" if you face any issues.

# Check if cargo is installed
if ! command -v cargo &> /dev/null
then
    echo "Cargo could not be found. Please install Rust."
    exit 1
fi

# Check if solana CLI is installed
if ! command -v solana &> /dev/null
then
    echo "Solana CLI could not be found. Please install Solana."
    exit 1
fi


# Build
cargo build-sbf --manifest-path=./program/Cargo.toml --sbf-out-dir=./program/target/so

# Deploy
solana program deploy ./program/target/so/create_account_program.so
