# Using token account data in transfer hook

This examples shows you how you can use the token account data in the transfer
hook. This is useful if you for example want to use the token account owner as a
seed for a PDA.

When creating the ExtraAccountMeta you can use the data of any account as an
extra seed. In this case we want to derive a counter account from the token
account owner and the string 'counter'.

This is how you set it up in the InitializeExtraAccountMetaList trait.

```rust
// Define extra account metas to store on extra_account_meta_list account
impl<'info> InitializeExtraAccountMetaList<'info> {
    pub fn extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        Ok(
            vec![
                ExtraAccountMeta::new_with_seeds(
                    &[
                        Seed::Literal {
                            bytes: b"counter".to_vec(),
                        },
                        Seed::AccountData { account_index: 0, data_index: 32, length: 32 },
                    ],
                    false, // is_signer
                    true // is_writable
                )?
            ]
        )
    }
}
```

Lets look at the token account struct to understand how the account data is
stored.This is how a token account looks like. So we can take 32 bytes at
position 32 to 64 as the owner of the token account which is at 'account_index:
0'.

```rust
/// Account data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Account {
    /// The mint associated with this account
    pub mint: Pubkey,
    /// The owner of this account.
    pub owner: Pubkey,
    /// The amount of tokens this account holds.
    pub amount: u64,
    pub delegate: COption<Pubkey>,
    pub state: AccountState,
    pub is_native: COption<u64>,
    pub delegated_amount: u64,
    pub close_authority: COption<Pubkey>,
}
```

Now we need to put the extra account metas in the InitializeExtraAccountMetaList
struct.

```rust
#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        init,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        space = ExtraAccountMetaList::size_of(
            InitializeExtraAccountMetaList::extra_account_metas()?.len()
        )?,
        payer = payer
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(init, seeds = [b"counter", payer.key().as_ref()], bump, payer = payer, space = 16)]
    pub counter_account: Account<'info, CounterAccount>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
```

and then to the remaining accounts of the transfer hook accounts.

```rust
#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(token::mint = mint, token::authority = owner)]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(token::mint = mint)]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: source token account owner, can be SystemAccount or PDA owned by another program
    pub owner: UncheckedAccount<'info>,
    /// CHECK: ExtraAccountMetaList Account,
    #[account(seeds = [b"extra-account-metas", mint.key().as_ref()], bump)]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    #[account(seeds = [b"counter", owner.key().as_ref()], bump)]
    pub counter_account: Account<'info, CounterAccount>,
}
```

In the client this account is auto generated and you can use it like this.

```rust
const transferInstructionWithHelper =
await createTransferCheckedWithTransferHookInstruction(
    connection,
    sourceTokenAccount,
    mint.publicKey,
    destinationTokenAccount,
    wallet.publicKey,
    amountBigInt,
    decimals,
    [],
    "confirmed",
    TOKEN_2022_PROGRAM_ID
);
```

The helper function is resolving the account automatically from the
ExtraAccounts data account. How the account would be resolved in the client is
like this:

```js
const [counterPDA] = PublicKey.findProgramAddressSync(
  [Buffer.from("counter"), wallet.publicKey.toBuffer()],
  program.programId,
);
```

The counter PDA in this case i created in the initialize hook. If you want this
to work for every account you need to add some function on your website to
create this account before hand probably.
