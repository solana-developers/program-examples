import { Account, Pubkey, Result, Signer, u8 } from "@solanaturbine/poseidon";

export default class CreateSystemAccountProgram {
  static PROGRAM_ID = new Pubkey(
    "J3h2xRJr7i3dUiLsPu9ZhFGKkNnnxvWAvRNXdKUx5wvi"
  );

  // Method to initialize a new system account
  createSystemAccount(account: AccountState, owner: Signer): Result {
    console.log("Program invoked. Creating a system account...");

    // Generate a new account using a derived address and initialize it
    account.derive(["account"]).init();

    // Assign the provided signer as the account's owner
    account.owner = owner.key;

    // Store the bump seed used for generating the derived account address
    account.accountBump = account.getBump();

    console.log("Account created succesfully.");
  }
}

export interface AccountState extends Account {
  owner: Pubkey; // Public key that owns the account
  accountBump: u8; // Bump seed used in address derivation
}
