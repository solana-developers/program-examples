#!/bin/bash

# This script is for quick building & deploying of the program.
# It also serves as a reference for the commands used for building & deploying Solana programs.
# Run this bad boy with "bash cicd.sh" or "./cicd.sh"

cargo build-bpf --bpf-out-dir=./target/so
echo "Hand:"
solana program deploy ./target/so/hand.so | grep "Program Id:"
echo "Lever:"
solana program deploy ./target/so/lever.so | grep "Program Id:"

# D2E39tDWxnndmW3QvYVzmm2gU2kvr9Zv2ywBLAmq8Fw3
# 9qprjFZKZbBeRGNT3vgPW5xpPFqXjNxKZWybUy7mYNBw