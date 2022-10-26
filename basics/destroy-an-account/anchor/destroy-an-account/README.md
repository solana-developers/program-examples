# Destroy an Account

1. We're creating a `PDA` using [create_user.rs](programs/destroy-an-account/src/instructions/create_user.rs)
   instruction.


2. We're closing it using [destroy_user.rs](programs/destroy-an-account/src/instructions/destroy_user.rs)
   instruction, which uses `Anchor` `AccoutClose` `trait`. 


3. In our test [destroy-an-account.ts](tests/destroy-an-account.ts) we're using `fetchNullable` since we expect 
   the account to be `null` prior to creation and after closing. 