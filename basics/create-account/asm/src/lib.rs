#[cfg(test)]
mod tests {
    use mollusk_svm::program;
    use mollusk_svm::{result::Check, Mollusk};
    use solana_account::Account;
    use solana_address::Address;
    use solana_instruction::{AccountMeta, Instruction};
    use solana_native_token::LAMPORTS_PER_SOL;

    const BASE_LAMPORTS: u64 = 10 * LAMPORTS_PER_SOL;

    #[test]
    fn test_hello_world() {
        let program_id_keypair_bytes =
            std::fs::read("deploy/create-account-asm-program-keypair.json").unwrap()[..32]
                .try_into()
                .expect("slice with incorrect length");
        let program_id = Address::new_from_array(program_id_keypair_bytes);
        let (system_program, system_account) = program::keyed_account_for_system_program();

        let payer_pubkey = Address::new_unique();
        let payer_account = Account::new(BASE_LAMPORTS, 0, &system_program);

        let new_account_pubkey = Address::new_unique();
        let new_account_account = Account::new(0, 0, &system_program);

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[],
            vec![
                AccountMeta::new(payer_pubkey, true),
                AccountMeta::new(new_account_pubkey, true),
                AccountMeta::new_readonly(system_program, false),
            ],
        );

        let mollusk = Mollusk::new(&program_id, "deploy/create-account-asm-program");

        let result = mollusk.process_and_validate_instruction(
            &instruction,
            &[
                (payer_pubkey, payer_account),
                (new_account_pubkey, new_account_account),
                (system_program, system_account.clone()),
            ],
            &[Check::success()],
        );
        assert!(!result.program_result.is_err());
    }
}
