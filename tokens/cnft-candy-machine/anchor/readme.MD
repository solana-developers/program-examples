# cNFT Candy Machine

This example demonstrates how to a candy machine for cNFTs.

In this example, a user will be able to create a cNFT candy machine. That candy machine will have different properties:
- Addresses can be added to the candy machine whitelist
- A token can be specified as a "whitelist token", allowing users to mint a cNFT by burning a speciific token
- The user will be able to pay in SOL and / or a specific SPL token

---

## Let's walk through the architecture:

A cNFT candy machine config account consists of:

```rust
#[account]
pub struct Config {
    pub authority: Pubkey,
    pub allow_list: Vec<AllowListStruct>,
    pub allow_mint: Option<Pubkey>,
    pub collection: Pubkey,
    pub total_supply: u32,
    pub current_supply: u32,
    pub price_sol: Option<u64>,
    pub price_spl: Option<u64>,
    pub spl_address: Option<Pubkey>,
    pub status: TreeStatus,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum TreeStatus {
    Inactive,
    Private,
    Public,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub struct AllowListStruct {
    pub user: Pubkey,
    pub amount: u8,
}
```

### In this state account, we will store:

- authority: The person who is starting the cNFT candy machine, and will be able to perform changes.

- allow_list: A vector of ´AllowListStruct´ which will contain the whitelisted addresses and the allowed amount for each address

- allow_mint: The address of the "whitelist token"

- collection: The collection address

- total_supply: The total supply of the collection

- current_supply: The current minted amount

- price_sol: The price to mint in SOL

- price_spl: The price to mint in SPL

- spl_address: The SPL token allowed to mint

- status: The current status of the config (It can be Inactive, Public or Private)

- bump: Since our config account will be a PDA (Program Derived Address), we will store the bump of the account

We use InitSpace derive macro to implement the space trait that will calculate the amount of space that our account will use on-chain (without taking the anchor discriminator into consideration)

---

### The user will be able to create new cNFT config accounts. For that, we create the following context:

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        seeds = [b"config", authority.key().as_ref()],
        bump,
        space = Config::INIT_SPACE
    )]
    pub config: Box<Account<'info, Config>>,
    pub allow_mint: Option<Account<'info, Mint>>,
    #[account(
        init,
        payer = authority,
        seeds = [b"collection", config.key().as_ref()],
        bump,
        mint::decimals = 0,
        mint::authority = config,
        mint::freeze_authority = config,
    )]
    pub collection: Account<'info, Mint>,
    /// CHECK: Tree Config chcecks will be performed by the Bubblegum Program
    #[account(mut)]
    pub tree_config: UncheckedAccount<'info>,
    /// CHECK: Unitialized Merkle Tree Account. Initialization will be performed by the Bubblegum Program 
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECK: SPL NOOP Program checked by the corresponding address
    #[account(address = SPL_NOOP_ID)]
    pub log_wrapper: UncheckedAccount<'info>,
    /// CHECK: Bubblegum Program checked by the corresponding address
    #[account(address = BUBBLEGUM_ID)]
    pub bubblegum_program: UncheckedAccount<'info>,
    /// CHECK: SPL Account Compression Program checked by the corresponding address
    #[account(address = SPL_ACCOUNT_COMPRESSION_ID)]
    pub compression_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
```

Let´s have a closer look at the accounts that we are passing in this context:

- authority: Will be the person starting the cNFT config account. He will be a signer of the transaction, and we mark his account as mutable as we will be deducting lamports from this account

- config: Will be the state account that we will initialize and the authority will be paying for the initialization of the account.
We derive the config PDA from the byte representation of the word "config" and the reference of the authority public key. Anchor will calculate the canonical bump (the first bump that throes that address out of the ed25519 eliptic curve) and save it for us in a struct

- allow_mint: The mint that can be used as whitelist

- collection: We will initialize a mint account that will be used as the collection address. This account will be derived from the byte representation of the word "collection" and the config account address

- tree_config: The tree config account. This is will be checked by the bubblegum program

- merkle_tree: The unitiliazed markle tree account. This initialization will be performed by the bubblegum program

- log_wrapper: SPL Noop program account

- bubblegum_program: Bubblegum program account

- compression_program: SPL account compression program account

- system_proram: System program which is responsible for initializing any new account

- token_program and associated_token_program: We are creating new ATAs

### We then implement some functionality for our Initialize context:

```rust
impl<'info> Initialize<'info> {
    pub fn init_config(&mut self, total_supply: u32, price_sol: Option<u64>, price_spl: Option<u64>, spl_address: Option<Pubkey>, bumps: &InitializeBumps) -> Result<()> {
        // Check if there is a mint in the allow mint account and return the key or None
        let allow_mint = match self.allow_mint.clone() {
            Some(value) => Some(value.key()),
            None => None,
        };

        // Check if there is a price and address for the SPL token and return the key or None
        let (price_spl, spl_address) = match price_spl.is_some() && spl_address.is_some() {
            true => (price_spl, spl_address),
            false => {
                // If one is true and the other is false, return an error
                require!(price_spl.is_none() && spl_address.is_none(), CustomError::InvalidSPLSettings);
                // If both are false, return None
                (None, None)
            },
        };

        self.config.set_inner(
            // Initialize the config account
            Config {
                authority: self.authority.key(),
                allow_list: vec![],
                allow_mint,
                collection: self.collection.key(),
                total_supply,
                current_supply: 0,
                price_sol,
                price_spl,
                spl_address,
                status: TreeStatus::Private,
                bump: bumps.config, 
            },
        );
        Ok(())
    }

    pub fn init_tree(&mut self, max_depth: u32, max_buffer_size: u32) -> Result<()> {
        // Create the seeds for the CPI call
        let seeds = &[
            &b"config"[..], 
            &self.authority.key.as_ref(),
            &[self.config.bump],
        ];
        let signer_seeds = &[&seeds[..]];
        
        // Accounts for CPI calls
        let bubblegum_program = &self.bubblegum_program.to_account_info();
        let tree_config = &self.tree_config.to_account_info();
        let merkle_tree = &self.merkle_tree.to_account_info();
        let tree_creator = &self.config.to_account_info();
        let payer = &self.authority.to_account_info();
        let log_wrapper = &self.log_wrapper.to_account_info();
        let compression_program = &self.compression_program.to_account_info();
        let system_program = &self.system_program.to_account_info();

        // CPI call to create the tree config
        CreateTreeConfigCpiBuilder::new(bubblegum_program)
            .tree_config(tree_config)
            .merkle_tree(merkle_tree)
            .payer(payer)
            .tree_creator(tree_creator)
            .log_wrapper(log_wrapper)
            .compression_program(compression_program)
            .system_program(system_program)
            .max_depth(max_depth)
            .max_buffer_size(max_buffer_size)
            .public(false)
            .invoke_signed(signer_seeds)?;          
        
        Ok(())
    }
}
```

In here, we basically just set the data of our config (max supply, the price in SOL and / or SPL, the allowed SPL token) and initialize our Tree Config by performing a CPI to the bubblegum program.
The authority will be able to customize the candy machine by setting a desired allow mint, a whitelist, and different payment methods (SOL and / or specific SPL tokens)

---

### The authority of the config account will be able to add addresses to the allow list

```rust
#[derive(Accounts)]
pub struct AllowList<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config", authority.key().as_ref()],
        bump = config.bump,
        realloc = Config::INIT_SPACE + (config.allow_list.len() * AllowListStruct::INIT_SPACE) + AllowListStruct::INIT_SPACE,
        realloc::payer = authority,
        realloc::zero = true,
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}
```

In this context, we are passing all the accounts needed to add a user to the allow list:

- authority: Authority of the config account

- config: An initialized config account. This account space will be reallocated by adding the length of the the ´AllowListStruct´ in order to store the new added user

- system account: The system program is the program responsible for performing the reallocation operqtion

### We then implement some functionality for our AllowList context:

```rust
impl<'info> AllowList<'info> {
    pub fn add(&mut self, user: Pubkey, amount: u8) -> Result<()> {
        // Add the user to the allow list
        self.config.allow_list.push(
            AllowListStruct{
                user,
                amount,
            }
        );
        Ok(())
    }
}
```
In here, we just add the the user and the allow amount to the allow list vector

---

### The authority of the config account will be able to change the status of the candy machine

```rust
#[derive(Accounts)]
pub struct SetTreeStatus<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config", authority.key().as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
}
```

In this context, we are passing all the accounts needed for the authority to change the candy machine status:

- authority: authority of the config account

- config: An initialized config account

### We then implement some functionality for our SetTreeStatus context:

```rust
impl<'info> SetTreeStatus<'info> {
    pub fn set_tree_status(&mut self, status: TreeStatus) -> Result<()> {
        // Set the tree status
        self.config.status = status;
        Ok(())
    }
}
```

In this implementation, we just update the status of the config account


---

### Users will be able to mint cNFTs

```rust
#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub authority: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"config", authority.key().as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub allow_mint: Option<Account<'info, Mint>>,
    #[account(mut)]
    pub allow_mint_ata: Option<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"collection", config.key().as_ref()],
        bump,
    )]
    pub collection: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            collection.key().as_ref()
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub collection_metadata: Account<'info, MetadataAccount>,
    #[account(
        mut,
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            collection.key().as_ref(),
            b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub collection_edition: Account<'info, MasterEditionAccount>,
    /// CHECK: Tree Config account that will be checked by the Bubblegum Program
    #[account(mut)]
    pub tree_config: UncheckedAccount<'info>,
    pub leaf_owner: SystemAccount<'info>,
    /// CHECK: Merkle Tree account that will be checked by the Bubblegum Program
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECK: SPL NOOP Program checked by the corresponding address
    #[account(address = SPL_NOOP_ID)]
    pub log_wrapper: UncheckedAccount<'info>,
    /// CHECK: Bubblegum Program checked by the corresponding address
    #[account(address = BUBBLEGUM_ID)]
    pub bubblegum_program: UncheckedAccount<'info>,
    /// CHECK: SPL Account Compression Program checked by the corresponding address
    #[account(address = SPL_ACCOUNT_COMPRESSION_ID)]
    pub compression_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub metadata_program: Program<'info, Metadata>,
}
```

In this context, we are passing all the accounts needed for a user to mint a cNFT:

- user: The address of the person that is claiming the cNFT

- authority: The authority of the config account

- config: The config account derived from the authority address

- allow_mint: An optional account with the allow mint

- allow_mint_ata: An optional account with the user's allow mint associated token account (ATA)

- collection: The cNFT collection mint account

- collection_metadata: The cNFT collection metadata account

- collection_edition: The cNFT collection master edition account

- tree_config: the tree config account. This is will be checked by the bubblegum program

- merkle_tree: the unitiliazed markle tree account. This initialization will be performed by the bubblegum program

- log_wrapper: SPL Noop program account

- bubblegum_program: Bubblegum program account

- compression_program: SPL account compression program account

- system_proram: We will be transfering lamports

- token_program: We will be burning and transfering SPL tokens

- metadata_program: The MPL metadata program address

### We then implement some functionality for our MintNFT context:

```rust
impl<'info> MintNFT<'info> {
    pub fn mint_cnft(&mut self, name: String, symbol: String, uri: String, pay_sol: bool, remaining_accounts: &[AccountInfo<'info>]) -> Result<()> {

        // Check if the Candy Machine is active
        require!(self.config.status != TreeStatus::Inactive, CustomError::CandyMachineInactive);

        // Check if the Candy Machine is private
        if self.config.status == TreeStatus::Private {
            // Check if there is an Allow Mint account and Allow Mint ATA account
            if let (Some(allow_mint), Some(allow_mint_ata)) = (&self.allow_mint, &self.allow_mint_ata) {

                // Check if the Allow Mint account is the same as the one in the config
                require!(allow_mint.key() == self.config.allow_mint.unwrap(), CustomError::InvalidAllowMint);

                // Check if the Allow Mint ATA account belongs to the user
                let ata_address = get_associated_token_address(&self.user.key(), &allow_mint.key());
                require!(ata_address == self.allow_mint_ata.as_ref().unwrap().key(), CustomError::InvalidAllowMintATA);

                // Burn the Allow Mint token
                let cpi_program = self.token_program.to_account_info();
                let cpi_accounts = Burn {
                    mint: allow_mint.to_account_info(),
                    from: allow_mint_ata.to_account_info(),
                    authority: self.user.to_account_info(),
                };
                let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
                burn(cpi_context, 1 * 10_u32.pow(allow_mint.decimals as u32) as u64)?;
            }
            else {
                // If the Candy Machine is private and there is no Allow Mint account, check if the user is in the Allow List
                self.config.allow_list.iter().find(|x| x.user == self.user.key()).ok_or(CustomError::UserNotAllowed)?;

                // Check if the user has already claimed
                let user_struct = self.config.allow_list.iter_mut().find(|x| x.user == self.user.key()).unwrap();      
                if user_struct.amount == 0 {
                    return Err(CustomError::AlreadyClaimed.into());
                }
                // Decrease the allowed amount of the user
                user_struct.amount -= 1;    
            }
        }

        // Create signer seeds for the CPI calls
        let seeds = &[
            &b"config"[..], 
            &self.authority.key.as_ref(),
            &[self.config.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // CPI call to the Bubblegum Program to mint the cNFT
        MintToCollectionV1CpiBuilder::new(&self.bubblegum_program.to_account_info())
            .tree_config(&self.tree_config.to_account_info())
            .leaf_owner(&self.user.to_account_info())
            .leaf_delegate(&self.user)
            .merkle_tree(&self.merkle_tree.to_account_info())
            .payer(&self.user.to_account_info())
            .tree_creator_or_delegate(&self.config.to_account_info())
            .collection_authority(&self.config.to_account_info())
            .collection_authority_record_pda(None)
            .collection_mint(&self.collection.to_account_info())
            .collection_metadata(&self.collection_metadata.to_account_info())
            .collection_edition(&self.collection_edition.to_account_info())
            .bubblegum_signer(&self.config.to_account_info())
            .log_wrapper(&self.log_wrapper.to_account_info())
            .compression_program(&self.compression_program.to_account_info())
            .token_metadata_program(&self.metadata_program.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .metadata(
                MetadataArgs {
                    name,
                    symbol,
                    uri,
                    creators: vec![],
                    seller_fee_basis_points: 0,
                    primary_sale_happened: false,
                    is_mutable: false,
                    edition_nonce: Some(0),
                    uses: None,
                    collection: Some(Collection {
                        verified: true,
                        key: self.collection.key(),
                    }),
                    token_program_version: TokenProgramVersion::Original,
                    token_standard: Some(TokenStandard::NonFungible),
                }
            )
        .invoke_signed(signer_seeds)?;

        // Check if the user wants to pay in SOL or SPL and if there is a price in SOL or SPL. Return an error if the settings are invalid
        match pay_sol {
            // If the user wants to pay in SOL, check if there is a price in SOL. 
            // If there is, transfer the SOL to the authority, otherwise check if there is a price in SPL and return an error if there is
            true => match self.config.price_sol.is_some() {
                true => self.transfer_sol()?,
                false => require!(self.config.price_spl.is_none(), CustomError::InvalidSPLSettings),
            },
            // If the user wants to pay in SPL, check if there is a price in SPL and an SPL address. 
            // If there is, transfer the SPL to the authority, otherwise check if there is a price in SOL and return an error if there is
            false => match self.config.price_spl.is_some() && self.config.spl_address.is_some() {
                true => self.transfer_spl(remaining_accounts)?,
                false => require!(self.config.price_sol.is_none(), CustomError::InvalidSPLSettings),
            },
        }

        // Increase the current supply
        self.config.current_supply += 1;

        // If the total supply is equal to the current supply, close the account
        if self.config.current_supply >= self.config.total_supply {
            self.close_account()?;
        }

        Ok(())
    }

    pub fn transfer_sol(&mut self) -> Result<()> {
        // Transfer the SOL to the authority
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.authority.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_context, self.config.price_sol.unwrap())
    }

    pub fn transfer_spl(&mut self, remaining_accounts: &[AccountInfo<'info>]) -> Result<()> {
        // Check if there are 2 remaining accounts
        if remaining_accounts.len() != 2 {
            return Err(CustomError::InvalidRemainingAccounts.into());
        }

        // Get the expected ATA accounts
        let expected_from_ata = get_associated_token_address(&self.user.key, &self.config.spl_address.as_ref().unwrap());
        let expected_to_ata = get_associated_token_address(&self.authority.key, &self.config.spl_address.as_ref().unwrap());

        // Check if the first remaining accounts are is the expected source ATA
        require_keys_eq!(remaining_accounts[0].key(), expected_from_ata, CustomError::InvalidSourceRemainingAccount);

        // Check if the second remaining account is the expected destination ATA
        require_keys_eq!(remaining_accounts[1].key(), expected_to_ata, CustomError::InvalidDestinationRemainingAccount);


        // Create the transfer instruction
        let transfer_tokens_instruction = spl_transfer(
            &self.token_program.key,
            &remaining_accounts[0].key(),
            &remaining_accounts[1].key(),
            &self.user.key(),
            &[&self.user.key()],
            self.config.price_spl.unwrap(),
        )?;
        
        // Collect the required accounts for the transfer
        let required_accounts_for_transfer = [
            remaining_accounts[0].to_account_info().clone(),
            remaining_accounts[1].to_account_info().clone(),
            self.user.to_account_info().clone(),
        ];
        
        // Invoke the transfer instruction
        invoke(
            &transfer_tokens_instruction,
            &required_accounts_for_transfer,
        )?;

        Ok(())
    }

    pub fn close_account(&mut self) -> Result<()> {
        // Close the config account and transfer the rent lamports to the authority
        **self.authority.lamports.borrow_mut() = self.authority.lamports().checked_add(self.config.to_account_info().lamports()).unwrap();
        **self.config.to_account_info().lamports.borrow_mut() = 0;
        
        Ok(())
    }
}
```

In here, all the minting magic will happen. A check will be performed to check if the candy machine status is not inactive and if it is private.
If its active and private, we will chack if there is any allow mint configured. There are two possible outcomes from this situation:
- If there is an allow mint configured, we will burn one token of the allow mint from the user that wants to mint
- If there is no allow mint configured, we will check if the user that wants to mint is in the allow list (whitelist) and update his number of "allowed mints"

After the previous checks, a cNFT will be minted to the user by performing a CPI to the Metaplex Bubblegum Program
In case that the status of the machine is public, no checks will be performed regarding allow mint and allow list and a cNFT will be minted to the user

The candy machine can be customized to receive payments in SOL and / or a specific SPL token.
The user will be able to choose with each he would like to pay. Checks will be performed (check if the desired payment is supported, perform proper accounts checks, etc) and, if all checks are successful, the user will be charged accordingly
