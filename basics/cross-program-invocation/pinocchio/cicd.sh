#!/bin/bash

# This script is for quick building & deploying of the program.
# It also serves as a reference for the commands used for building & deploying Solana programs.
# Run this bad boy with "bash cicd.sh" or "./cicd.sh"

cargo build-sbf --bpf-out-dir=./target/so
echo "Hand:"
solana program deploy ./target/so/hand.so | grep "Program Id:"
echo "Lever:"
solana program deploy ./target/so/lever.so | grep "Program Id:"
