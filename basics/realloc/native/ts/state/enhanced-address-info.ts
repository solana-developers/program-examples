import { Buffer } from 'node:buffer';
import * as borsh from 'borsh';

export class EnhancedAddressInfo {
  name: string;
  house_number: number;
  street: string;
  city: string;
  state: string;
  zip: number;

  constructor(props: {
    name: string;
    house_number: number;
    street: string;
    city: string;
    state: string;
    zip: number;
  }) {
    this.name = props.name;
    this.house_number = props.house_number;
    this.street = props.street;
    this.city = props.city;
    this.state = props.state;
    this.zip = props.zip;
  }

  toBase58() {
    return borsh.serialize(EnhancedAddressInfoSchema, this).toString();
  }

  toBuffer() {
    return Buffer.from(borsh.serialize(EnhancedAddressInfoSchema, this));
  }

  static fromBuffer(buffer: Buffer) {
    return borsh.deserialize(EnhancedAddressInfoSchema, EnhancedAddressInfo, buffer);
  }
}

export const EnhancedAddressInfoSchema = new Map([
  [
    EnhancedAddressInfo,
    {
      kind: 'struct',
      fields: [
        ['name', 'string'],
        ['house_number', 'u8'],
        ['street', 'string'],
        ['city', 'string'],
        ['state', 'string'],
        ['zip', 'u32'],
      ],
    },
  ],
]);
