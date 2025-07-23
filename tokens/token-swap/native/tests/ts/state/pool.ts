import { PublicKey } from "@solana/web3.js";
import * as borsh from 'borsh';

export class Pool {
    amm: PublicKey;
    mint_a: PublicKey;
    mint_b: PublicKey;

    constructor(props: {
        amm: PublicKey,
        mint_a: PublicKey,
        mint_b: PublicKey,
    }) {
        this.amm = props.amm;
        this.mint_a = props.mint_a;
        this.mint_b = props.mint_b;
    }

    static fromBuffer(buffer: Buffer) {
        const decoded = borsh.deserialize(PoolSchema, Pool, buffer);
        decoded.amm = new PublicKey(decoded.amm);
        decoded.mint_a = new PublicKey(decoded.mint_a);
        decoded.mint_b = new PublicKey(decoded.mint_b);
        return decoded
    }
}

export const PoolSchema = new Map([
    [Pool, {
        kind: 'struct',
        fields: [
            ['amm', ['u8', 32]],
            ['mint_a', ['u8', 32]],
            ['mint_b', ['u8', 32]],
        ]
    }]
]);
