#! /bin/bash

# get root directory
ROOT_DIR=$(cd $(dirname $0)/.. && pwd)

ROOT_ANCHOR_DIR=$ROOT_DIR/anchor

cd $ROOT_ANCHOR_DIR

solana-test-validator --reset --ledger $ROOT_ANCHOR_DIR/test-ledger --quiet &

# wait for validator to start
sleep 5

anchor deploy --provider.cluster localnet
