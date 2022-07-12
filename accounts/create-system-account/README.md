# Create Account

:wrench: We're going to create a Solana account. :wrench:   
   
This account is going to be a **system account** - meaning it will be owned by the System Program. In short, this means only the System Program will be allowed to modify it's data.   
   
In this example, this account will simply hold some SOL.

### Links:
- [Solana Cookbook - How to Create a System Account](https://solanacookbook.com/references/accounts.html#how-to-create-a-system-account)
- [Rust Docs - solana_program::system_instruction::create_account](https://docs.rs/solana-program/latest/solana_program/system_instruction/fn.create_account.html)