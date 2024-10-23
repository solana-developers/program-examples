#!/bin/bash

cargo build-sbf --manifest-path=./program/Cargo.toml --bpf-out-dir=./program/target/so
solana program deploy ./program/target/so/program.so
