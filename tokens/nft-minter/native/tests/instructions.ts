import * as borsh from 'borsh';

class Assignable {
  constructor(properties) {
    for (const [key, value] of Object.entries(properties)) {
      this[key] = value;
    }
  }
}

export enum NftMinterInstruction {
  Create = 0,
  Mint = 1,
}

export class CreateTokenArgs extends Assignable {
  toBuffer() {
    return Buffer.from(borsh.serialize(CreateTokenArgsSchema, this));
  }
}
const CreateTokenArgsSchema = new Map([
  [
    CreateTokenArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['nft_title', 'string'],
        ['nft_symbol', 'string'],
        ['nft_uri', 'string'],
      ],
    },
  ],
]);

export class MintToArgs extends Assignable {
  toBuffer() {
    return Buffer.from(borsh.serialize(MintToArgsSchema, this));
  }
}
const MintToArgsSchema = new Map([
  [
    MintToArgs,
    {
      kind: 'struct',
      fields: [['instruction', 'u8']],
    },
  ],
]);
