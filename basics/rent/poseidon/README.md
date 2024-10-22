# Rent (Poseidon)

Poseidon does not have a method equivalent to Rent::get(). Instead, the execution of account initialization is transpiled into the [account(init)] attribute of Anchor.

Within Anchor, the following process occurs: 
- Rent::get().minimum_balance(account_size) is used to calculate the amount of lamports required for the account to be rent-exempt. 
- That amount is then deducted from the payer's account (owner) and deposited into the newly created account.

### Required amount for rent (exempt account)
The amount of lamports required for rent is determined based on the size of the account and the current rent rates set by the Solana network. 

Therefore, in this example, an account with the necessary amount for rent is first created. In the test code, the amount (determined based on the size of the account) is then verified to ensure it is correct.