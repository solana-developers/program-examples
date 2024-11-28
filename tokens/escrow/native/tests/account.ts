import { PublicKey } from '@solana/web3.js';
import * as borsh from 'borsh';

export class OfferAccount {
  id: bigint;
  maker: Uint8Array;
  token_mint_a: Uint8Array;
  token_mint_b: Uint8Array;
  token_b_wanted_amount: bigint;
  bump: number;

  constructor(offer: OfferRaw) {
    this.id = offer.id;
    this.maker = offer.maker;
    this.token_b_wanted_amount = offer.token_b_wanted_amount;
    this.token_mint_a = offer.token_mint_a;
    this.token_mint_b = offer.token_mint_b;
    this.bump = this.bump;
  }

  toBuffer() {
    return Buffer.from(borsh.serialize(OfferSchema, this));
  }

  static fromBuffer(buffer: Uint8Array) {
    return borsh.deserialize(OfferSchema, OfferAccount, Buffer.from(buffer));
  }

  toData(): Offer {
    return {
      id: this.id,
      maker: new PublicKey(this.maker),
      token_mint_a: new PublicKey(this.token_mint_a),
      token_mint_b: new PublicKey(this.token_mint_b),
      token_b_wanted_amount: this.token_b_wanted_amount,
      bump: this.bump,
    };
  }
}

const OfferSchema = new Map([
  [
    OfferAccount,
    {
      kind: 'struct',
      fields: [
        ['id', 'u64'],
        ['maker', [32]],
        ['token_mint_a', [32]],
        ['token_mint_b', [32]],
        ['token_b_wanted_amount', 'u64'],
        ['bump', 'u8'],
      ],
    },
  ],
]);

type OfferRaw = {
  id: bigint;
  maker: Uint8Array;
  token_mint_a: Uint8Array;
  token_mint_b: Uint8Array;
  token_b_wanted_amount: bigint;
  bump: number;
};

type Offer = {
  id: bigint;
  maker: PublicKey;
  token_mint_a: PublicKey;
  token_mint_b: PublicKey;
  token_b_wanted_amount: bigint;
  bump: number;
};
