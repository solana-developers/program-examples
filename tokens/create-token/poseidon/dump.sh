#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Mainnet URL
MAINNET_URL="https://api.mainnet-beta.solana.com"

# Function to check Solana installation
check_solana_installation() {
    if ! command -v solana &> /dev/null; then
        echo "Error: Solana CLI is not installed or not in PATH"
        return 1
    fi

    if ! solana --version &> /dev/null; then
        echo "Error: Solana CLI is available but returned an error"
        return 1
    fi

    return 0
}

# Function to check and set Solana network to mainnet
check_and_set_mainnet() {
    # Get current cluster
    current_cluster=$(solana config get | grep "RPC URL" | awk '{print $3}')

    # If not on mainnet, switch to mainnet
    if [[ "$current_cluster" != "$MAINNET_URL" ]]; then
        echo "Switching Solana network to mainnet..."
        solana config set --url "$MAINNET_URL"
    fi
}

# Function to check if Solana test validator is running
check_test_validator() {
    # Check if solana-test-validator process is running
      if ! pgrep -f "solana-test-validator" &> /dev/null; then
        echo "Error: Solana test validator is not running."
        echo "Please start Solana test validator in a new terminal in this directory:"
        echo "cd $(pwd) && solana-test-validator"
        return 1
    fi

    return 0
}

# Main script
main() {
    # Check if Solana is installed
    if ! check_solana_installation; then
        echo "Solana check failed"
        exit 1
    fi

    # Check and set to mainnet if needed
    check_and_set_mainnet

    # Check if test validator is running
    if ! check_test_validator; then
        exit 1
    fi

    # Dump Solana program
    if ! solana program dump metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s mpl_token_metadata.so; then
        echo "Failed to dump Solana program"
        exit 1
    fi

    echo "Solana program dumped successfully"
}

# Run the main function
main

