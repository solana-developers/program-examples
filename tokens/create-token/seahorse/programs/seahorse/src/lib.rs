#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod dot;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token, TokenAccount},
};

use dot::program::*;
use std::{cell::RefCell, rc::Rc};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod seahorse_util {
    use super::*;

    #[cfg(feature = "pyth-sdk-solana")]
    pub use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed};
    use std::{collections::HashMap, fmt::Debug, ops::Deref};

    pub struct Mutable<T>(Rc<RefCell<T>>);

    impl<T> Mutable<T> {
        pub fn new(obj: T) -> Self {
            Self(Rc::new(RefCell::new(obj)))
        }
    }

    impl<T> Clone for Mutable<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<T> Deref for Mutable<T> {
        type Target = Rc<RefCell<T>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: Debug> Debug for Mutable<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl<T: Default> Default for Mutable<T> {
        fn default() -> Self {
            Self::new(T::default())
        }
    }

    impl<T: Clone> Mutable<Vec<T>> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index >= 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    impl<T: Clone, const N: usize> Mutable<[T; N]> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index >= 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    #[derive(Clone)]
    pub struct Empty<T: Clone> {
        pub account: T,
        pub bump: Option<u8>,
    }

    #[derive(Clone, Debug)]
    pub struct ProgramsMap<'info>(pub HashMap<&'static str, AccountInfo<'info>>);

    impl<'info> ProgramsMap<'info> {
        pub fn get(&self, name: &'static str) -> AccountInfo<'info> {
            self.0.get(name).unwrap().clone()
        }
    }

    #[derive(Clone, Debug)]
    pub struct WithPrograms<'info, 'entrypoint, A> {
        pub account: &'entrypoint A,
        pub programs: &'entrypoint ProgramsMap<'info>,
    }

    impl<'info, 'entrypoint, A> Deref for WithPrograms<'info, 'entrypoint, A> {
        type Target = A;

        fn deref(&self) -> &Self::Target {
            &self.account
        }
    }

    pub type SeahorseAccount<'info, 'entrypoint, A> =
        WithPrograms<'info, 'entrypoint, Box<Account<'info, A>>>;

    pub type SeahorseSigner<'info, 'entrypoint> = WithPrograms<'info, 'entrypoint, Signer<'info>>;

    #[derive(Clone, Debug)]
    pub struct CpiAccount<'info> {
        #[doc = "CHECK: CpiAccounts temporarily store AccountInfos."]
        pub account_info: AccountInfo<'info>,
        pub is_writable: bool,
        pub is_signer: bool,
        pub seeds: Option<Vec<Vec<u8>>>,
    }

    #[macro_export]
    macro_rules! seahorse_const {
        ($ name : ident , $ value : expr) => {
            macro_rules! $name {
                () => {
                    $value
                };
            }

            pub(crate) use $name;
        };
    }

    #[macro_export]
    macro_rules! assign {
        ($ lval : expr , $ rval : expr) => {{
            let temp = $rval;

            $lval = temp;
        }};
    }

    #[macro_export]
    macro_rules! index_assign {
        ($ lval : expr , $ idx : expr , $ rval : expr) => {
            let temp_rval = $rval;
            let temp_idx = $idx;

            $lval[temp_idx] = temp_rval;
        };
    }

    pub(crate) use assign;

    pub(crate) use index_assign;

    pub(crate) use seahorse_const;
}

#[program]
mod seahorse {
    use super::*;
    use seahorse_util::*;
    use std::collections::HashMap;

    #[derive(Accounts)]
    pub struct InitTokenAccount<'info> {
        # [account (init , payer = signer , seeds = ["token-account1" . as_bytes () . as_ref () , signer . key () . as_ref ()] , bump , token :: mint = mint , token :: authority = signer)]
        pub new_token_account: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub mint: Box<Account<'info, Mint>>,
        #[account(mut)]
        pub signer: Signer<'info>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
        pub token_program: Program<'info, Token>,
    }

    pub fn init_token_account(ctx: Context<InitTokenAccount>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let new_token_account = Empty {
            account: SeahorseAccount {
                account: &ctx.accounts.new_token_account,
                programs: &programs_map,
            },
            bump: ctx.bumps.get("new_token_account").map(|bump| *bump),
        };

        let mint = SeahorseAccount {
            account: &ctx.accounts.mint,
            programs: &programs_map,
        };

        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        init_token_account_handler(new_token_account.clone(), mint.clone(), signer.clone());

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct InitTokenMint<'info> {
        # [account (init , payer = signer , seeds = ["token-mint" . as_bytes () . as_ref () , signer . key () . as_ref ()] , bump , mint :: decimals = 0 , mint :: authority = signer)]
        pub new_token_mint: Box<Account<'info, Mint>>,
        #[account(mut)]
        pub signer: Signer<'info>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
        pub token_program: Program<'info, Token>,
    }

    pub fn init_token_mint(ctx: Context<InitTokenMint>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let new_token_mint = Empty {
            account: SeahorseAccount {
                account: &ctx.accounts.new_token_mint,
                programs: &programs_map,
            },
            bump: ctx.bumps.get("new_token_mint").map(|bump| *bump),
        };

        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        init_token_mint_handler(new_token_mint.clone(), signer.clone());

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct UseTokenMint<'info> {
        #[account(mut)]
        pub mint: Box<Account<'info, Mint>>,
        #[account(mut)]
        pub recipient: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub signer: Signer<'info>,
        pub token_program: Program<'info, Token>,
    }

    pub fn use_token_mint(ctx: Context<UseTokenMint>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let mint = SeahorseAccount {
            account: &ctx.accounts.mint,
            programs: &programs_map,
        };

        let recipient = SeahorseAccount {
            account: &ctx.accounts.recipient,
            programs: &programs_map,
        };

        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        use_token_mint_handler(mint.clone(), recipient.clone(), signer.clone());

        return Ok(());
    }
}
