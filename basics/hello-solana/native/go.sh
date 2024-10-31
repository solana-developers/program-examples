cargo build-sbf --manifest-path=./program/Cargo.toml --sbf-out-dir=./program/target/so
solana program deploy ./program/target/so/program.so
