use pinocchio::{account::AccountView, entrypoint, Address, ProgramResult};

use crate::instructions::{
    claim, close_bracket, emit_event, finalize, init_config, lock, post_goals, post_result, refresh_score,
    submit_bracket, WorldCupInstruction,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Address,
    accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    match WorldCupInstruction::from_bytes(instruction_data)? {
        WorldCupInstruction::InitConfig(data) => init_config::process(accounts, &data),
        WorldCupInstruction::SubmitBracket(data) => submit_bracket::process(accounts, &data),
        WorldCupInstruction::Lock => lock::process(accounts),
        WorldCupInstruction::PostResult(data) => post_result::process(accounts, &data),
        WorldCupInstruction::PostGoals(data) => post_goals::process(accounts, &data),
        WorldCupInstruction::RefreshScore => refresh_score::process(accounts),
        WorldCupInstruction::Finalize => finalize::process(accounts),
        WorldCupInstruction::Claim => claim::process(accounts),
        WorldCupInstruction::CloseBracket => close_bracket::process(accounts),
        WorldCupInstruction::EmitEvent => emit_event::process(program_id, accounts),
    }
}
