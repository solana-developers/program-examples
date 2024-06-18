import { Buffer } from 'node:buffer';
import * as borsh from 'borsh';

export class AddressInfo {
  name: string;
  house_number: number;
  street: string;
  city: string;

  constructor(props: {
    name: string;
    house_number: number;
    street: string;
    city: string;
  }) {
    this.name = props.name;
    this.house_number = props.house_number;
    this.street = props.street;
    this.city = props.city;
  }

  toBase58() {
    return borsh.serialize(AddressInfoSchema, this).toString();
  }

  toBuffer() {
    return Buffer.from(borsh.serialize(AddressInfoSchema, this));
  }

  static fromBuffer(buffer: Buffer) {
    return borsh.deserialize(AddressInfoSchema, AddressInfo, buffer);
  }
}

export const AddressInfoSchema = new Map([
  [
    AddressInfo,
    {
      kind: 'struct',
      fields: [
        ['name', 'string'],
        ['house_number', 'u8'],
        ['street', 'string'],
        ['city', 'string'],
      ],
    },
  ],
]);
