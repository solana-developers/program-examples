use quasar_lang::{
    cpi::{BufCpiCall, InstructionAccount},
    prelude::*,
};

/// Accounts for the hand program's pull_lever instruction.
/// The lever_program uses `Program<LeverProgram>` with a custom marker type
/// that implements `Id` — this lets Quasar verify the program address and
/// the executable flag during account parsing.
#[derive(Accounts)]
pub struct PullLever<'info> {
    #[account(mut)]
    pub power: &'info UncheckedAccount,
    pub lever_program: &'info Program<crate::LeverProgram>,
}

impl<'info> PullLever<'info> {
    #[inline(always)]
    pub fn pull_lever(&self, name: &str) -> Result<(), ProgramError> {
        log("Hand is pulling the lever!");

        // Build the switch_power instruction data for the lever program:
        //   [disc=1] [name: u32 len + bytes]
        // 128 bytes is enough for any reasonable name.
        let mut data = [0u8; 128];
        let name_bytes = name.as_bytes();
        let data_len = 1 + 4 + name_bytes.len();

        // Discriminator = 1 (switch_power)
        data[0] = 1;

        // Name string: u32 little-endian length prefix + bytes
        let len_bytes = (name_bytes.len() as u32).to_le_bytes();
        data[1] = len_bytes[0];
        data[2] = len_bytes[1];
        data[3] = len_bytes[2];
        data[4] = len_bytes[3];

        // Copy name bytes
        let mut i = 0;
        while i < name_bytes.len() {
            data[5 + i] = name_bytes[i];
            i += 1;
        }

        let power_view = self.power.to_account_view();
        let lever_view = self.lever_program.to_account_view();

        // Build CPI call with 1 account (power, writable, not a signer).
        let cpi = BufCpiCall::<1, 128>::new(
            lever_view.address(),
            [InstructionAccount::writable(power_view.address())],
            [power_view],
            data,
            data_len,
        );

        cpi.invoke()
    }
}
