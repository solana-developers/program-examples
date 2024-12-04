import { SYSVAR_RENT_PUBKEY, TransactionInstruction } from "@solana/web3.js";
import { PublicKey } from "@solana/web3.js";
import { AmmInstruction } from './';
import * as borsh from 'borsh';
import { SystemProgram } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";

export class CreatePoolArgs {
    instruction: AmmInstruction;

    constructor(props: { instruction: AmmInstruction }) {
        this.instruction = props.instruction;
    }

    toBuffer() {
        return Buffer.from(borsh.serialize(CreateAmmArgsSchema, this));
    }

    static fromBuffer(buffer: Buffer) {
        return borsh.deserialize(CreateAmmArgsSchema, CreatePoolArgs, buffer);
    }

}

export const CreateAmmArgsSchema = new Map([
    [
        CreatePoolArgs,
        {
            kind: 'struct',
            fields: [
                ['instruction', 'u8'],
            ],
        },
    ],
]);

export function createCreatePoolInstruction(amm: PublicKey, pool: PublicKey, poolAuthority: PublicKey, mintLiquidity: PublicKey, mintA: PublicKey, mintB: PublicKey, poolAccountA: PublicKey, poolAccountB: PublicKey, payer: PublicKey, programId: PublicKey): TransactionInstruction {
    const instructionObject = new CreatePoolArgs({
        instruction: AmmInstruction.CreatePool,
    });

    const ix = new TransactionInstruction({
        keys: [
            { pubkey: amm, isSigner: false, isWritable: true },
            { pubkey: pool, isSigner: false, isWritable: true },
            { pubkey: poolAuthority, isSigner: false, isWritable: false },
            { pubkey: mintLiquidity, isSigner: false, isWritable: true },
            { pubkey: mintA, isSigner: false, isWritable: false },
            { pubkey: mintB, isSigner: false, isWritable: false },
            { pubkey: poolAccountA, isSigner: false, isWritable: true },
            { pubkey: poolAccountB, isSigner: false, isWritable: true },
            { pubkey: payer, isSigner: true, isWritable: true },
            { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
            { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
            { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        ],
        programId: programId,
        data: instructionObject.toBuffer(),
    });

    return ix;
}