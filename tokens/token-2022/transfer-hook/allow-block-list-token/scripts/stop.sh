#! /bin/bash

#
ps ax | grep solana-test-validator | grep -v grep | awk '{print $1}' | xargs kill -9