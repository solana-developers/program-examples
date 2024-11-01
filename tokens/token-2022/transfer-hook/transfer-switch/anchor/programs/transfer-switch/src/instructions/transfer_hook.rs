use {
    crate::{error::TransferError, state::TransferSwitch},
    anchor_lang::prelude::*,
    anchor_spl::{
        token_2022::spl_token_2022::{
            extension::{
                transfer_hook::TransferHookAccount, BaseStateWithExtensionsMut,
                PodStateWithExtensionsMut,
            },
            pod::PodAccount,
        },
        token_interface::Mint,
    },
};

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct TransferHook<'info> {
    /// CHECK: Sender token account
    #[account()]
    pub source_token_account: UncheckedAccount<'info>,

    /// The mint of the token transferring
    #[account()]
    pub token_mint: InterfaceAccount<'info, Mint>,

    /// CHECK: Recipient token account
    #[account()]
    pub receiver_token_account: UncheckedAccount<'info>,

    /// CHECK: the transfer sender
    #[account()]
    pub wallet: UncheckedAccount<'info>,

    /// CHECK: extra account metas
    #[account(
        seeds = [b"extra-account-metas", token_mint.key().as_ref()],
        bump,
    )]
    pub extra_account_metas_list: UncheckedAccount<'info>,

    /// sender transfer switch
    #[account(
        seeds=[wallet.key().as_ref()],
        bump,
    )]
    pub wallet_switch: Account<'info, TransferSwitch>,
}

impl<'info> TransferHook<'info> {
    pub fn assert_switch_is_on(&mut self) -> Result<()> {
        if !self.wallet_switch.on {
            return err!(TransferError::SwitchNotOn);
        }
        Ok(())
    }

    pub fn assert_is_transferring(&self) -> Result<()> {
        let source_token_info = self.source_token_account.to_account_info();
        let mut account_data_ref = source_token_info.try_borrow_mut_data()?;
        let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;
        let account_extension = account.get_extension_mut::<TransferHookAccount>()?;

        if !bool::from(account_extension.transferring) {
            return err!(TransferError::IsNotCurrentlyTransferring);
        }

        Ok(())
    }
}
