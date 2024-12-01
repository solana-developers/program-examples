/**
 * @title Counter Program
 * @description A Solana program that manages a simple counter using the Turbine framework
 * @dev This program allows initialization and increment operations on counter accounts
 */

import { Account, Pubkey, Result, Signer, u8, u64 } from '@solanaturbine/poseidon';

/**
 * @notice Main program class implementing counter functionality
 * @dev Uses Turbine framework for Solana program development
 */
export default class Counter {
  /**
   * @notice Program ID for the Counter Program
   * @dev Deployed program address on Solana
   */
  static PROGRAM_ID = new Pubkey('3dhKkikKk112wEVdNr69Q2eEXSwU3MivfTNxauQsTjJP');

  /**
   * @notice Initializes a new counter account
   * @dev Creates a PDA account using 'counter' and authority's public key as seeds
   *
   * @param authority The signer who will have authority over the counter
   * @param counter The counter account to be initialized
   * @return Result Success/failure of the initialization
   */
  initialize(authority: Signer, counter: CounterAccount): Result {
    // Derive PDA for counter account using seeds
    counter.derive(['counter', authority.key]).init(authority);

    // Initialize counter state
    counter.count = new u64(0);
    counter.bump = counter.getBump();
  }

  /**
   * @notice Increments the counter value by 1
   * @dev Verifies counter PDA using stored bump seed for efficiency
   *
   * @param authority The signer authorized to increment the counter
   * @param counter The counter account to be incremented
   * @return Result Success/failure of the increment operation
   */
  increment(authority: Signer, counter: CounterAccount): Result {
    // Verify counter PDA using stored bump seed
    counter.deriveWithBump(['counter', authority.key], counter.bump);
    // Increment counter value
    counter.count = counter.count.add(1);
  }
}

/**
 * @notice Interface defining the structure of a Counter account
 * @dev Extends the base Account type from Turbine framework
 */
export interface CounterAccount extends Account {
  /** Current value of the counter */
  count: u64;
  /** Bump seed used in PDA derivation */
  bump: u8;
}
