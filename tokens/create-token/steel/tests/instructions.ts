import * as borsh from "borsh";
import { BN } from "bn.js";

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
  decimals: Buffer;

  constructor(name: string, symbol: string, uri: string, decimals: string) {
    this.instruction = SplMinterInstruction.Create;
    this.name = strToBytes(name, 32);
    this.symbol = strToBytes(symbol, 8);
    this.uri = strToBytes(uri, 64);
    this.decimals = strToBytes(decimals, 8);
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
