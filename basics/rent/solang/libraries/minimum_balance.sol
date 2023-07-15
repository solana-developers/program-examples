// SPDX-License-Identifier: Apache-2.0

// Disclaimer: This library provides functions for working with storage rent. Although it is production ready,
// it has not been audited for security, so use it at your own risk.

// This is the Solidity version of the rust module rent:
// https://github.com/solana-labs/solana/blob/master/sdk/program/src/rent.rs
// As rent is currently not implemented on Solana, only the minimum balance is required.

/// Default rental rate in lamports/byte-year.
///
/// This calculation is based on:
/// - 10^9 lamports per SOL
/// - $1 per SOL
/// - $0.01 per megabyte day
/// - $3.65 per megabyte year
uint64 constant DEFAULT_LAMPORTS_PER_BYTE_YEAR = 1_000_000_000 / 100 * 365 / (1024 * 1024);

/// Default amount of time (in years) the balance has to include rent for the
/// account to be rent exempt.
uint64 constant DEFAULT_EXEMPTION_THRESHOLD = 2;

/// Account storage overhead for calculation of base rent.
///
/// This is the number of bytes required to store an account with no data. It is
/// added to an accounts data length when calculating [`Rent::minimum_balance`].
uint64 constant ACCOUNT_STORAGE_OVERHEAD = 128;

/// Minimum balance due for rent-exemption of a given account data size.
function minimum_balance(uint64 data_len) pure returns (uint64) {
    return ((ACCOUNT_STORAGE_OVERHEAD + data_len) * DEFAULT_LAMPORTS_PER_BYTE_YEAR)
        * DEFAULT_EXEMPTION_THRESHOLD;
}