import { TransactionInstruction } from "@solana/web3.js";
import { PublicKey } from "@solana/web3.js";
import { AmmInstruction } from './';
import * as borsh from 'borsh';
import { SystemProgram } from "@solana/web3.js";

export class CreateAmmArgs {
    instruction: AmmInstruction;
    fee: number; // u16

    constructor(props: { instruction: AmmInstruction; fee: number }) {
        this.instruction = props.instruction;
        this.fee = props.fee;
    }

    toBuffer() {
        return Buffer.from(borsh.serialize(CreateAmmArgsSchema, this));
    }

    static fromBuffer(buffer: Buffer) {
        return borsh.deserialize(CreateAmmArgsSchema, CreateAmmArgs, buffer);
    }

}

export const CreateAmmArgsSchema = new Map([
    [
        CreateAmmArgs,
        {
            kind: 'struct',
            fields: [
                ['instruction', 'u8'],
                ['fee', 'u16']
            ],
        },
    ],
]);

export function createCreateAmmInstruction(amm: PublicKey, admin: PublicKey, payer: PublicKey, programId: PublicKey, fee: number): TransactionInstruction {
    const instructionObject = new CreateAmmArgs({
        instruction: AmmInstruction.CreateAmm,
        fee: fee,
    });

    const ix = new TransactionInstruction({
        keys: [
            { pubkey: amm, isSigner: false, isWritable: true },
            { pubkey: admin, isSigner: false, isWritable: false },
            { pubkey: payer, isSigner: true, isWritable: true },
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        programId: programId,
        data: instructionObject.toBuffer(),
    });

    return ix;
}
