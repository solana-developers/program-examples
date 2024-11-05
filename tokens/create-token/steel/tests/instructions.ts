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

export enum CreateTokenInstruction {
  Create = 0,
}

export class CreateTokenArgs {
  instruction: number;
  name: Buffer;
  symbol: Buffer;
  uri: Buffer;
  decimals: number;

  constructor(name: string, symbol: string, uri: string, decimals: number) {
    this.instruction = CreateTokenInstruction.Create;
    this.name = strToBytes(name, 32);
    this.symbol = strToBytes(symbol, 8);
    this.uri = strToBytes(uri, 64);
    this.decimals = decimals;
  }

  toBuffer(): Buffer {
    // Added 1 byte for decimals to the total buffer size
    const buffer = Buffer.alloc(1 + 32 + 8 + 64 + 1);
    let offset = 0;

    // Write instruction
    buffer.writeUInt8(this.instruction, offset);
    offset += 1;

    // Write name
    this.name.copy(buffer, offset);
    offset += 32;

    // Write symbol
    this.symbol.copy(buffer, offset);
    offset += 8;

    // Write uri
    this.uri.copy(buffer, offset);
    offset += 64;

    // Write decimals
    buffer.writeUInt8(this.decimals, offset);

    return buffer;
  }
}
