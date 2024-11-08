/// Helper for creating steel instruction with borsh.
/// Not required, but helpful with typing and CPI
///
#[macro_export]
macro_rules! borsh_instruction {
    ($discriminator_name:ident, $struct_name:ident) => {
        impl $crate::Discriminator for $struct_name {
            fn discriminator() -> u8 {
                $discriminator_name::$struct_name as u8
            }
        }

        // adds discriminator to the instruction data, helpful with cpis etc.
        impl $struct_name {
            pub fn to_bytes(&self) -> Result<Vec<u8>, solana_program::program_error::ProgramError> {
                let instruction_vec = borsh::to_vec(self)?;
                Ok([
                    [$discriminator_name::$struct_name as u8].to_vec(),
                    instruction_vec,
                ]
                .concat())
            }
        }
    };
}
