// tests/types.ts
import type { PublicKey } from "@solana/web3.js";

export interface ProgramTestContext {
  // biome-ignore lint/suspicious/noExplicitAny: TODO: we should fix this, but we also will move these test to LiteSVM for Anchor 1.0
  connection: any;
  programs: {
    programId: PublicKey;
    program: string;
  }[];
  grantLamports: (address: PublicKey, amount: number) => Promise<void>;
  terminate: () => Promise<void>;
}

export interface UserAccount {
  authority: PublicKey;
  ethereumAddress: number[];
}
