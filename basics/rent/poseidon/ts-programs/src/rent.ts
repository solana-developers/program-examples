import { Account, Pubkey, Result, Signer, u32 } from "@solanaturbine/poseidon";

export default class RentProgram {
  static PROGRAM_ID = new Pubkey(
    "9M9xaYvQeFcBf2eHsJJPJWaQB34yUHKgcumskCCKM875"
  );

  // Create a new system account
  createSystemAccount(account: AccountState, owner: Signer): Result {
    account.derive(["account"]).init();
    account.owner = owner.key;
    // Account size = 32 + 4 + 8(account header size) = 44 bytes
    account.space = 44 as unknown as u32;
  }
}

export interface AccountState extends Account {
  owner: Pubkey; // Owner (32 bytes)
  space: u32; // Account size (4 bytes)
}
