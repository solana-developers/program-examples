import { BN } from 'bn.js';
import * as borsh from 'borsh';

class Assignable {
  constructor(properties) {
    for (const [key, value] of Object.entries(properties)) {
      this[key] = value;
    }
  }
}

// Helper function to pad strings to fixed length buffers
function strToBytes(str: string, length: number): Buffer {
  const buffer = Buffer.alloc(length);
  buffer.write(str);
  return buffer;
}

export enum SplMinterInstruction {
  Create = 0,
  Mint = 1,
}

export class CreateTokenArgs {
  instruction: number;
  name: Buffer;
  symbol: Buffer;
  uri: Buffer;

  constructor(name: string, symbol: string, uri: string) {
    this.instruction = SplMinterInstruction.Create;
    this.name = strToBytes(name, 32);
    this.symbol = strToBytes(symbol, 8);
    this.uri = strToBytes(uri, 64);
  }

  toBuffer(): Buffer {
    const buffer = Buffer.alloc(1 + 32 + 8 + 64);
    let offset = 0;

    buffer.writeUInt8(this.instruction, offset);
    offset += 1;

    this.name.copy(buffer, offset);
    offset += 32;
    this.symbol.copy(buffer, offset);
    offset += 8;
    this.uri.copy(buffer, offset);

    return buffer;
  }
}

export class MintToArgs {
  instruction: number;
  quantity: BN;

  constructor(quantity: number) {
    this.instruction = SplMinterInstruction.Mint;
    this.quantity = new BN(quantity);
  }

  toBuffer(): Buffer {
    const buffer = Buffer.alloc(9); // 1 byte for instruction + 8 bytes for u64 quantity
    let offset = 0;

    // Write instruction
    buffer.writeUInt8(this.instruction, offset);
    offset += 1;

    // Write quantity as u64 LE (8 bytes)
    this.quantity.toBuffer('le', 8).copy(buffer, offset);

    return buffer;
  }
}
