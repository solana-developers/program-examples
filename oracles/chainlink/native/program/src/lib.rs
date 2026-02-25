use chainlink_solana::v2::read_feed_v2;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

struct Decimal {
    pub value: i128,
    pub decimals: u32,
}

impl Decimal {
    pub fn new(value: i128, decimals: u32) -> Self {
        Decimal { value, decimals }
    }
}

impl std::fmt::Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut scaled_val = self.value.to_string();
        if scaled_val.len() <= self.decimals as usize {
            scaled_val.insert_str(
                0,
                &vec!["0"; self.decimals as usize - scaled_val.len()].join(""),
            );
            scaled_val.insert_str(0, "0.");
        } else {
            scaled_val.insert(scaled_val.len() - self.decimals as usize, '.');
        }
        f.write_str(&scaled_val)
    }
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey, // Ignored
    accounts: &[AccountInfo],
    _instruction_data: &[u8], // Ignored
) -> ProgramResult {
    msg!("Chainlink Price Feed Consumer entrypoint");

    let accounts_iter = &mut accounts.iter();

    // This is the account of the price feed data to read from
    let feed_account = next_account_info(accounts_iter)?;

    // Read the feed data directly from the account (v2 SDK)
    let result = read_feed_v2(
        feed_account.try_borrow_data()?,
        feed_account.owner.to_bytes(),
    )
    .map_err(|_| solana_program::program_error::ProgramError::InvalidAccountData)?;

    // Get the latest round data
    let round = result
        .latest_round_data()
        .ok_or(solana_program::program_error::ProgramError::InvalidAccountData)?;

    let description = result.description();
    let decimals = result.decimals();

    // Convert description bytes to string
    let description_str = std::str::from_utf8(&description)
        .unwrap_or("Unknown")
        .trim_end_matches('\0');

    let decimal_print = Decimal::new(round.answer, u32::from(decimals));
    msg!("{} price is {}", description_str, decimal_print);

    Ok(())
}
