import * as borsh from 'borsh';

class Assignable {
  constructor(properties) {
    for (const [key, value] of Object.entries(properties)) {
      this[key] = value;
    }
  }
}

export enum EscrowInstruction {
  InitEscrow = 0,
  Exchange = 1,
}

export class InitEscrowArgs extends Assignable {
  toBuffer() {
    return Buffer.from(borsh.serialize(InitEscrowArgsSchema, this));
  }
}

const InitEscrowArgsSchema = new Map([
  [
    InitEscrowArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['amount', 'u64'],
      ],
    },
  ],
]);

export class ExchangeArgs extends Assignable {
  toBuffer() {
    return Buffer.from(borsh.serialize(ExchangeArgsSchema, this));
  }
}

const ExchangeArgsSchema = new Map([
  [
    ExchangeArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['amount', 'u64'],
      ],
    },
  ],
]);
