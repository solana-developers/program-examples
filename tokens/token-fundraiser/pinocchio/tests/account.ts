// Mirrors the on-chain `Fundraiser` layout in `program/src/state.rs`.
export const FundraiserSchema = {
  struct: {
    maker: { array: { type: "u8", len: 32 } },
    mint_to_raise: { array: { type: "u8", len: 32 } },
    amount_to_raise: "u64",
    current_amount: "u64",
    time_started: "i64",
    duration: "u16",
    bump: "u8",
  },
};

export type FundraiserRaw = {
  maker: Uint8Array;
  mint_to_raise: Uint8Array;
  amount_to_raise: bigint;
  current_amount: bigint;
  time_started: bigint;
  duration: number;
  bump: number;
};

// Mirrors the on-chain `Contributor` layout in `program/src/state.rs`.
export const ContributorSchema = {
  struct: {
    amount: "u64",
    bump: "u8",
  },
};

export type ContributorRaw = {
  amount: bigint;
  bump: number;
};
