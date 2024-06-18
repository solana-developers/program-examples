// From https://github.com/solana-labs/solana/blob/a94920a4eadf1008fc292e47e041c1b3b0d949df/sdk/program/src/system_instruction.rs
export const systemProgramErrors = [
  'an account with the same address already exists',

  'account does not have enough SOL to perform the operation',

  'cannot assign account to this program id',

  'cannot allocate account data of this length',

  'length of requested seed is too long',

  'provided address does not match addressed derived from seed',

  'advancing stored nonce requires a populated RecentBlockhashes sysvar',

  'stored nonce is still in recent_blockhashes',

  'specified nonce does not match stored nonce',
];
