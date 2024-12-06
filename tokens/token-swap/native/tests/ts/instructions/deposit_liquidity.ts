import { SystemProgram, TransactionInstruction } from "@solana/web3.js";
import { AmmInstruction } from ".";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import * as borsh from 'borsh';

export class DepositLiquidityArgs {
    instruction: AmmInstruction;
    amount_a: number; // u64
    amount_b: number; // u64

    constructor(props: { instruction: AmmInstruction; amount_a: number; amount_b: number }) {
        this.instruction = props.instruction;
        this.amount_a = props.amount_a;
        this.amount_b = props.amount_b;
    }

    toBuffer() {
        return Buffer.from(borsh.serialize(DepositLiquidityArgsSchema, this));
    }
}

export const DepositLiquidityArgsSchema = new Map([
    [
        DepositLiquidityArgs,
        {
            kind: 'struct',
            fields: [
                ['instruction', 'u8'],
                ['amount_a', 'u64'],
                ['amount_b', 'u64'],
            ]
        }
    ]
]);

export function createDepositLiquidityInstruction(pool, poolAuthority, depositor, mintLiquidity, mintA, mintB, poolAccountA, poolAccountB, depositorAccountLiquidity, depositorAccountA, depositorAccountB, amountA, amountB, programId) {
    const instructionObject = new DepositLiquidityArgs({
        instruction: AmmInstruction.DepositLiquidity,
        amount_a: amountA,
        amount_b: amountB,
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