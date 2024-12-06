import { TransactionInstruction } from "@solana/web3.js";
import { AmmInstruction } from ".";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import * as borsh from 'borsh';

export class WithdrawLiquidityArgs {
    instruction: AmmInstruction;
    amount: number; // u64

    constructor(props: { instruction: AmmInstruction; amount: number }) {
        this.instruction = props.instruction;
        this.amount = props.amount;
    }

    toBuffer() {
        return Buffer.from(borsh.serialize(WithdrawLiquidityArgsSchema, this));
    }
}

export const WithdrawLiquidityArgsSchema = new Map([
    [
        WithdrawLiquidityArgs,
        {
            kind: 'struct',
            fields: [
                ['instruction', 'u8'],
                ['amount', 'u64'],
            ]
        }
    ]
]);

export function createWithdrawLiquidityInstruction(pool, poolAuthority, depositor, mintLiquidity, mintA, mintB, poolAccountA, poolAccountB, depositorAccountLiquidity, depositorAccountA, depositorAccountB, amount, programId) {
    const instructionObject = new WithdrawLiquidityArgs({
        instruction: AmmInstruction.WithdrawLiquidity,
        amount: amount,
    });

    const ix = new TransactionInstruction({
        keys: [
            { pubkey: pool, isSigner: false, isWritable: false },
            { pubkey: poolAuthority, isSigner: false, isWritable: false },
            { pubkey: depositor, isSigner: true, isWritable: false },
            { pubkey: mintLiquidity, isSigner: false, isWritable: true },
            { pubkey: mintA, isSigner: false, isWritable: false },
            { pubkey: mintB, isSigner: false, isWritable: false },
            { pubkey: poolAccountA, isSigner: false, isWritable: true },
            { pubkey: poolAccountB, isSigner: false, isWritable: true },
            { pubkey: depositorAccountLiquidity, isSigner: false, isWritable: true },
            { pubkey: depositorAccountA, isSigner: false, isWritable: true },
            { pubkey: depositorAccountB, isSigner: false, isWritable: true },
            { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        ],
        programId: programId,
        data: instructionObject.toBuffer(),
    })

    return ix;
}