import { TransactionInstruction } from "@solana/web3.js";
import { AmmInstruction } from ".";
import * as borsh from 'borsh';
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";


export class SwapExactTokensForTokensArgs {
    instruction: AmmInstruction;
    swap_a: number; // u8 representing bool
    input_amount: number; // u64
    min_output_amount: number; // u64

    constructor(props: { instruction: AmmInstruction; swap_a: boolean; input_amount: number; min_output_amount: number }) {
        this.instruction = props.instruction;
        this.swap_a = props.swap_a ? 1 : 0;
        this.input_amount = props.input_amount;
        this.min_output_amount = props.min_output_amount;
    }

    toBuffer() {
        return Buffer.from(borsh.serialize(SwapExactTokensForTokensArgsSchema, this));
    }
}

export const SwapExactTokensForTokensArgsSchema = new Map([
    [
        SwapExactTokensForTokensArgs,
        {
            kind: 'struct',
            fields: [
                ['instruction', 'u8'],
                ['swap_a', 'u8'],
                ['input_amount', 'u64'],
                ['min_output_amount', 'u64'],
            ]
        }
    ]
]);

export function createSwapExactTokensForTokensInstruction(amm, pool, poolAuthority, trader, mintA, mintB, poolAccountA, poolAccountB, traderAccountA, traderAccountB, swapA, inputAmount, minOutputAmount, programId) {
    const instructionObject = new SwapExactTokensForTokensArgs({
        instruction: AmmInstruction.SwapExactTokensForTokens,
        swap_a: swapA,
        input_amount: inputAmount,
        min_output_amount: minOutputAmount,
    });

    const ix = new TransactionInstruction({
        keys: [
            { pubkey: amm, isSigner: false, isWritable: false },
            { pubkey: pool, isSigner: false, isWritable: false },
            { pubkey: poolAuthority, isSigner: false, isWritable: false },
            { pubkey: trader, isSigner: true, isWritable: false },
            { pubkey: mintA, isSigner: false, isWritable: false },
            { pubkey: mintB, isSigner: false, isWritable: false },
            { pubkey: poolAccountA, isSigner: false, isWritable: true },
            { pubkey: poolAccountB, isSigner: false, isWritable: true },
            { pubkey: traderAccountA, isSigner: false, isWritable: true },
            { pubkey: traderAccountB, isSigner: false, isWritable: true },
            { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        ],
        programId: programId,
        data: instructionObject.toBuffer(),
    })

    return ix;

}
