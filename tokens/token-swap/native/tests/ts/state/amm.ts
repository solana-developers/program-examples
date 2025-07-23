import { PublicKey } from "@solana/web3.js";
import * as borsh from 'borsh';

export class Amm {
    admin: PublicKey;
    fee: number;

    constructor(props: {
        admin: PublicKey,
        fee: number,
    }) {
        this.admin = props.admin;
        this.fee = props.fee;
    }

    static fromBuffer(buffer: Buffer) {
        const decoded = borsh.deserialize(AmmSchema, Amm, buffer);
        decoded.admin = new PublicKey(decoded.admin);
        return decoded
    }
}

export const AmmSchema = new Map([
    [Amm, {
        kind: 'struct',
        fields: [
            ['admin', ['u8', 32]],
            ['fee', 'u16'],
        ]
    }]
]);
