use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    CreateAmm = 0,
    CreatePool = 1,
    DepositLiquidity = 2,
    SwapExactTokens = 3,
    WithdrawLiquidity = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateAmm {
    pub id: [u8; 32],
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
pub struct SwapExactTokens {
    pub swap_a: u8,
    pub input_amount: [u8; 8],
    pub min_output_amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct WithdrawLiquidity {
    pub amount: [u8; 8],
}

instruction!(SteelInstruction, CreateAmm);
instruction!(SteelInstruction, CreatePool);
instruction!(SteelInstruction, DepositLiquidity);
instruction!(SteelInstruction, SwapExactTokens);
instruction!(SteelInstruction, WithdrawLiquidity);
