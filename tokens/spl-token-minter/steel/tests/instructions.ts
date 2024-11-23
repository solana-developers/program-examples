import * as borsh from 'borsh';

class Assignable {
  constructor(properties) {
    for (const [key, value] of Object.entries(properties)) {
      this[key] = value;
    }
  }
}

export enum MyInstruction {
  Create = 0,
  MintTo = 1,
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
        ['token_title', [32]],
        ['token_symbol', [10]],
        ['token_uri', [256]],
        ['decimals', 'u8'],
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
      fields: [
        ['instruction', 'u8'],
        ['quantity', 'u64'],
      ],
    },
  ],
]);
