# Token Swap Escrow

This anchor program creates an escrow to swap tokens. A maker deposits a certain amount of token X and asks for a certain amount of token Y in return. If you have any question please reach out to @andrescorreia on X / Twitter. 

## State:

The program only requires a single state named `Escrow`

### Escrow

This state will store all the information necessary to perform checks throughout the entire escrow process.

```rust
pub struct Escrow {
    pub seed: u64,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub amount: u64,
    pub bump: u8,
}
```

By storing `seed` and `bump`, we can verify that we are actually "accessing" the correct escrow account.
We store the Pubkey of `mint_x` and `mint_y` to later verify that both the tokens that are being deposited and withdraw are the correct ones.
The value `amount` is used to store the amount of tokens from `mint_y` requested by out "maker"

## Contexts:

### Make

The `Make` context is used to initialize the escrow account and to perform the initial deposit. 

We start by initializing our initializing the escrow account based on the maker pubkey and a seed passed as an instruction argument (allowing a maker to have multiple escrows). 

```rust
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        init,
        payer = maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = Escrow::INIT_SPACE,
        bump,
    )]
    pub escrow: Account<'info, Escrow>,
```

We also pass the address of the mints involved in the swap, in order to verify / initialize the `maker` associated token accounts used to perform the escrow swap 

```rust
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_x,
        associated_token::authority = escrow
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = maker,
    )]
    pub maker_ata_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_y,
        associated_token::authority = maker,
    )]
    pub maker_ata_y: InterfaceAccount<'info, TokenAccount>,
```

We then proceed and implement functionality for our `Make` context to populate our escrow account and deposit `mint_x` to the escrow vault with a CPI call.

```rust
impl<'info> Make<'info> {
    pub fn make(&mut self, seed: u64, amount: u64, bumps: &MakeBumps, deposit: u64) -> Result<()> {

        // Initialize escrow account
        self.escrow.seed = seed;
        self.escrow.mint_x = self.mint_x.key();
        self.escrow.mint_y = self.mint_y.key();
        self.escrow.amount = amount;
        self.escrow.bump = bumps.escrow;

        // Transfer deposit amount to vault
        self.transfer(deposit)
    }

    pub fn transfer(&mut self, deposit: u64) -> Result<()> {
        // Create CPI context
        let cpi_accounts = TransferChecked {
            from: self.maker_ata_x.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
            mint: self.mint_x.to_account_info(),
        };

        // Fetch CPI program
        let cpi_program = self.token_program.to_account_info();

        // Create CPI context
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Transfer deposit amount to vault by invoking transfer_checked
        transfer_checked(cpi_ctx, deposit, self.mint_x.decimals)
    }
}
```

### Take

The `Take` context is used to allow a user to "take" the escrow.

We start by validating the escrow account (by checking the account seeds, bump, and stored mint addresses). We also close the escrow account and send the rent back to the maker by using the constraint `close = maker`.

```rust
#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        has_one = mint_x,
        has_one = mint_y,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
```

We then perform checks and initializations for the needed associated token accounts

```rust
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_x,
        associated_token::authority = taker,
    )]
    pub taker_mint_x_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_y,
        associated_token::authority = taker,
    )]
    pub taker_mint_y_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = maker,
    )]
    pub maker_mint_y_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = escrow.mint_x,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
```

As functionality for our context, we start by transfering the amount of mint Y requested from the taker associated token account directly to the maker associated token account.
After that, we empty our mint X escrow vault to the taker associated token account

```rust
pub fn take(&mut self) -> Result<()> {

        // Transfer amount from taker to maker
        self.transfer(self.escrow.amount, false)?;
        self.transfer(self.vault.amount, true)
}

pub fn transfer(&mut self, amount: u64, is_x: bool) -> Result<()> {
    // Check if we are tranfering mint x or mint y
    match is_x {
        true => {
            // Create signer seeds
            let signer_seeds: [&[&[u8]];1] = [
                &[
                    b"escrow", 
                    self.maker.to_account_info().key.as_ref(), 
                    &self.escrow.seed.to_le_bytes()[..],
                    &[self.escrow.bump]
                ]
            ];

            // Create CPI accounts
            let cpi_accounts = TransferChecked {
                from: self.vault.to_account_info(),
                to: self.taker_mint_x_ata.to_account_info(),
                authority: self.escrow.to_account_info(),
                mint: self.mint_x.to_account_info(),
            };
            // Fetch CPI program
            let cpi_program = self.token_program.to_account_info();
            // Create CPI context
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);
            // Transfer tokens
            transfer_checked(cpi_ctx, amount, self.mint_x.decimals)
        }

        false => {
            // Create CPI accounts
            let cpi_accounts = TransferChecked {
                from: self.taker_mint_y_ata.to_account_info(),
                to: self.maker_mint_y_ata.to_account_info(),
                authority: self.taker.to_account_info(),
                mint: self.mint_y.to_account_info(),
            };
            // Fetch CPI program
            let cpi_program = self.token_program.to_account_info();
            // Create CPI context
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            // Transfer tokens
            transfer_checked(cpi_ctx, amount, self.mint_y.decimals)
        }
    }
}
```

Last step is to close our vault and send the rent back to the maker

```rust
pub fn close(&mut self) -> Result<()> {
    // Create signer seeds
    let signer_seeds: [&[&[u8]];1] = [
        &[
            b"escrow", 
            self.maker.to_account_info().key.as_ref(), 
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump]
        ]
    ];

    // Create CPI accounts
    let cpi_accounts = CloseAccount {
        account: self.vault.to_account_info(),
        destination: self.maker.to_account_info(),
        authority: self.escrow.to_account_info(),
    };
    // Fetch CPI program
    let cpi_program = self.token_program.to_account_info();
    // Create CPI context
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);
    // Close vault
    close_account(cpi_ctx)
}
```

### Refund

The refund context is used to allow the `maker` to cancel an escrow (if it has not been taken yet).

We start by making the usual checks (escrow account and associated token accounts). We also close the escrow account and send the rent back to the maker by using the constraint `close = maker`.

```rust
#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = maker
    )]
    pub maker_ata_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        has_one = mint_x,
        has_one = mint_y,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = escrow.mint_x,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}
```

We then implement functionality to empty the vault back to the maker

```rust
pub fn empty_vault(&mut self) -> Result<()> {
    // Create signer seeds
    let signer_seeds: [&[&[u8]];1] = [
        &[
            b"escrow", 
            self.maker.to_account_info().key.as_ref(), 
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump]
        ]
    ];

    // Create CPI accounts
    let cpi_accounts = TransferChecked {
        from: self.vault.to_account_info(),
        to: self.maker_ata_x.to_account_info(),
        authority: self.escrow.to_account_info(),
        mint: self.mint_x.to_account_info(),
    };
    // Fetch CPI program
    let cpi_program = self.token_program.to_account_info();
    // Create CPI context
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);
    // Transfer tokens
    transfer_checked(cpi_ctx, self.vault.amount, self.mint_x.decimals)
}
```

Finally, we close the escrow vault and send the rent back to the maker

```rust
pub fn close_vault(&mut self) -> Result<()> {
    // Create signer seeds
    let signer_seeds: [&[&[u8]];1] = [
        &[
            b"escrow", 
            self.maker.to_account_info().key.as_ref(), 
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump]
        ]
    ];

    // Create CPI accounts
    let cpi_accounts = CloseAccount {
        account: self.vault.to_account_info(),
        destination: self.maker.to_account_info(),
        authority: self.escrow.to_account_info(),
    };
    // Fetch CPI program
    let cpi_program = self.token_program.to_account_info();
    // Create CPI context
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);
    // Close vault
    close_account(cpi_ctx)
}
```
