#!/bin/bash

# This script is for quick building & deploying of the program.
# It also serves as a reference for the commands used for building & deploying Solana programs.
# Run this bad boy with "bash cicd.sh" or "./cicd.sh"

sbpf build --deploy-dir ./tests/fixtures/
solana program deploy ./tests/fixtures/checking-account-asm-program.so
