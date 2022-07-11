#!/bin/bash

# This script is for quick building & deploying of the program.
# It also serves as a reference for the commands used for building & deploying Solana programs.
# Run this bad boy with "bash cicd.sh" or "./cicd.sh"

cargo build-bpf --manifest-path=./program/Cargo.toml --bpf-out-dir=./target/so
solana program deploy /target/so/program.so