import * as borsh from 'borsh';

class Assignable {
  constructor(properties) {
    for (const [key, value] of Object.entries(properties)) {
      this[key] = value;
    }
  }
}

export enum NftMinterInstruction {
  Init = 0,
  Create = 1,
  Mint = 2,
}

export class InitArgs extends Assignable {
  toBuffer() {
    return Buffer.from(borsh.serialize(InitArgsSchema, this));
  }
}
const InitArgsSchema = new Map([
  [
    InitArgs,
    {
      kind: 'struct',
      fields: [['instruction', 'u8']],
    },
  ],
]);

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
