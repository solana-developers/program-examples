import { Account, Pubkey, type Result, Signer, u64 } from '@solanaturbine/poseidon';

export default class CheckingAccounts {
  // Check 1: The program ID check is automatically handled by Anchor
  static PROGRAM_ID = new Pubkey('8MWRHcfRvyUJpou8nD5oG7DmZ2Bmg99qBP8q5fZ5xJpg');

  // Initialize user_account Instruction
  initialize(
    // ACCOUNTS
    payer: Signer, // Check: Signer account verification
    user_account: UserAccountState,
    data: u64,
  ): Result {
    // CONTEXT

    // Check 2: Account initialization state is handled by Anchor's init constraint
    user_account.derive(['program']).init();

    user_account.user_data = data;
    user_account.authority = payer.key;
  }

  // Update user_account Instruction
  update(
    // ACCOUNTS
    authority: Signer,
    user_account: UserAccountState,
    new_data: u64,
  ): Result {
    // CONTEXT

    // Check 3: Ensures PDA matches the expected seeds
    // Check 4: Validates that the stored authority matches the signer
    user_account.derive(['program']).has([authority]).constraints([]);

    user_account.user_data = new_data;
  }
}

// STATE
export interface UserAccountState extends Account {
  user_data: u64;
  authority: Pubkey;
}
