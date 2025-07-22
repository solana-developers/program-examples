use anchor_lang::prelude::*;

use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};

use crate::AB_WALLET_SEED;




pub fn get_meta_list_size() -> Result<usize> {
    Ok(ExtraAccountMetaList::size_of(1).unwrap())
}

pub fn get_extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
    Ok(vec![
        // [5] ab_wallet for destination token account wallet
        ExtraAccountMeta::new_with_seeds(
            &[
                Seed::Literal {
                    bytes: AB_WALLET_SEED.to_vec(),
                },
                Seed::AccountData {
                    account_index: 2,
                    data_index: 32,
                    length: 32,
                },
            ],
            false,
            false,
        )?, // [2] destination token account
    ])
}
