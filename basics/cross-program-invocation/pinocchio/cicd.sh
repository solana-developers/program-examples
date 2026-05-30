#!/bin/bash

# Quick build script for the pinocchio CPI example.
# Builds both programs and emits .so files into tests/fixtures so bankrun can pick them up.

cargo build-sbf --manifest-path=./programs/hand/Cargo.toml --sbf-out-dir=./tests/fixtures
cargo build-sbf --manifest-path=./programs/lever/Cargo.toml --sbf-out-dir=./tests/fixtures
