#!/bin/bash

# This script is for quick building & deploying of the program.
# It also serves as a reference for the commands used for building & deploying Solana programs.
# Run this bad boy with "bash cicd.sh" or "./cicd.sh"

cargo build-sbf --manifest-path=./program/Cargo.toml
solana program deploy ./program/target/deploy/program.so
