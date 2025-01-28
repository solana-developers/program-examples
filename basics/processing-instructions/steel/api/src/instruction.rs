use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum ProcessingInstructionsInstruction {
    GoToThePark = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq)]
pub struct GoToTheParkData {
    pub name: [u8; 64],
    pub height: [u8; 8],
}

impl_to_bytes!(GoToTheParkData);

fn string_to_bytes(s: &str) -> [u8; 64] {
    let mut bytes = [0; 64];
    let s_bytes = s.as_bytes();
    let len = s_bytes.len().min(64);
    bytes[..len].copy_from_slice(&s_bytes[..len]);
    bytes
}

impl GoToTheParkData {
    pub fn new(name: String, height: u64) -> Self {
        Self {
            name: string_to_bytes(&name),
            height: height.to_le_bytes(),
        }
    }

    pub fn try_from_bytes(data: &[u8]) -> Result<&Self, ProgramError> {
        bytemuck::try_from_bytes(data).or(Err(ProgramError::InvalidInstructionData))
    }

    pub fn name(&self) -> String {
        String::from_utf8_lossy(&self.name)
            .trim_end_matches(char::from(0))
            .to_string()
    }

    pub fn height(&self) -> u64 {
        u64::from_le_bytes(self.height)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct GoToThePark {
    pub data: GoToTheParkData,
}

instruction!(ProcessingInstructionsInstruction, GoToThePark);
