use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum TokenSwapInstruction {
    CreateAmm = 0,
    CreatePool = 1,
    DepositLiquidity = 2,
    WithdrawLiquidity = 3,
    Swap = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateAmm {
    pub id: Pubkey,
    pub fee: [u8; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreatePool {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct DepositLiquidity {
    pub amount_a: [u8; 8],
    pub amount_b: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct WithdrawLiquidity {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Swap {
    pub swap_a: u8,
    pub input_amount: [u8; 8],
    pub min_output_amount: [u8; 8],
}

instruction!(TokenSwapInstruction, CreateAmm);
instruction!(TokenSwapInstruction, CreatePool);
instruction!(TokenSwapInstruction, DepositLiquidity);
instruction!(TokenSwapInstruction, WithdrawLiquidity);
instruction!(TokenSwapInstruction, Swap);
