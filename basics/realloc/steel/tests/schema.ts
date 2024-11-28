import { Buffer } from 'node:buffer';
import * as borsh from 'borsh';

export enum ReallocInstruction {
  Create = 0,
  Extend = 1,
  ZeroInit = 2,
}

export class AddressInfo {
  name: Uint8Array;
  house_number: number;
  street: Uint8Array;
  city: Uint8Array;

  constructor(info: {
    name: Uint8Array;
    house_number: number;
    street: Uint8Array;
    city: Uint8Array;
  }) {
    this.name = info.name;
    this.house_number = info.house_number;
    this.street = info.street;
    this.city = info.city;
  }

  toBuffer() {
    return Buffer.from(borsh.serialize(AddressInfoSchema, this));
  }

  static fromAccountData(buffer: Buffer) {
    const _accountData = Uint8Array.from(buffer).slice(8); // remove 8 byte discriminator

    return borsh.deserialize(AddressInfoSchema, AddressInfo, Buffer.from(_accountData));
  }

  static fromData(info: {
    name: string;
    house_number: number;
    street: string;
    city: string;
  }) {
    return new AddressInfo({
      name: Uint8Array.from(Buffer.from(info.name.padEnd(48, '\0'))),
      city: Uint8Array.from(Buffer.from(info.city.padEnd(48, '\0'))),
      street: Uint8Array.from(Buffer.from(info.street.padEnd(48, '\0'))),
      house_number: info.house_number,
    });
  }

  toData() {
    return {
      name: Buffer.from(this.name).toString(),
      city: Buffer.from(this.city).toString(),
      house_number: this.house_number,
      street: Buffer.from(this.street).toString(),
    };
  }
}

const AddressInfoSchema = new Map([
  [
    AddressInfo,
    {
      kind: 'struct',
      fields: [
        ['name', [48]], // Fixed array of 48 bytes
        ['house_number', 'u8'],
        ['street', [48]], // Fixed array of 48 bytes
        ['city', [48]], // Fixed array of 48 bytes
      ],
    },
  ],
]);

export class ExtendedAddressInfo {
  name: Uint8Array;
  house_number: number;
  street: Uint8Array;
  city: Uint8Array;
  state: Uint8Array;
  zip: number;

  constructor(info: {
    name: Uint8Array;
    house_number: number;
    street: Uint8Array;
    city: Uint8Array;
    state: Uint8Array;
    zip: number;
  }) {
    this.name = info.name;
    this.house_number = info.house_number;
    this.street = info.street;
    this.city = info.city;
    this.state = info.state;
    this.zip = info.zip;
  }

  static fromAccountData(buffer: Buffer) {
    const _accountData = Uint8Array.from(buffer).slice(8); // remove 8 byte discriminator

    return borsh.deserialize(ExtendedAddressInfoSchema, ExtendedAddressInfo, Buffer.from(_accountData));
  }

  toData() {
    return {
      name: Buffer.from(this.name).toString(),
      city: Buffer.from(this.city).toString(),
      house_number: this.house_number,
      street: Buffer.from(this.street).toString(),
      state: Buffer.from(this.state).toString(),
      zip: this.zip,
    };
  }
}

const ExtendedAddressInfoSchema = new Map([
  [
    ExtendedAddressInfo,
    {
      kind: 'struct',
      fields: [
        ['name', [48]], // Fixed array of 48 bytes
        ['house_number', 'u8'],
        ['street', [48]], // Fixed array of 48 bytes
        ['city', [48]], // Fixed array of 48 bytes
        ['state', [48]], // Fixed array of 48 bytes
        ['zip', 'u32'], // Fixed array of 48 bytes
      ],
    },
  ],
]);

export class AddressInfoExtender {
  state: Uint8Array;
  zip: number;

  constructor(info: { state: Uint8Array; zip: number }) {
    this.state = info.state;
    this.zip = info.zip;
  }

  toBuffer() {
    return Buffer.from(borsh.serialize(AddressInfoExtenderSchema, this));
  }

  static fromData(info: { state: string; zip: number }) {
    return new AddressInfoExtender({
      zip: info.zip,
      state: Uint8Array.from(Buffer.from(info.state.padEnd(48, '\0'))),
    });
  }
}

const AddressInfoExtenderSchema = new Map([
  [
    AddressInfoExtender,
    {
      kind: 'struct',
      fields: [
        ['state', [48]], // Fixed array of 48 bytes
        ['zip', 'u32'], // Fixed array of 48 bytes
      ],
    },
  ],
]);

export class WorkInfo {
  name: Uint8Array;
  position: Uint8Array;
  company: Uint8Array;
  years_employed: number;

  constructor(info: {
    name: Uint8Array;
    position: Uint8Array;
    company: Uint8Array;
    years_employed: number;
  }) {
    this.name = info.name;
    this.position = info.position;
    this.company = info.company;
    this.years_employed = info.years_employed;
  }

  toBuffer() {
    return Buffer.from(borsh.serialize(WorkInfoSchema, this));
  }

  static fromAccountData(buffer: Buffer) {
    const _accountData = Uint8Array.from(buffer).slice(8); // remove 8 byte discriminator

    return borsh.deserialize(WorkInfoSchema, WorkInfo, Buffer.from(_accountData));
  }

  static fromData(info: {
    name: string;
    position: string;
    company: string;
    years_employed: number;
  }) {
    return new WorkInfo({
      name: Uint8Array.from(Buffer.from(info.name.padEnd(48, '\0'))),
      position: Uint8Array.from(Buffer.from(info.position.padEnd(48, '\0'))),
      company: Uint8Array.from(Buffer.from(info.company.padEnd(48, '\0'))),
      years_employed: info.years_employed,
    });
  }

  toData() {
    return {
      name: Buffer.from(this.name).toString(),
      position: Buffer.from(this.position).toString(),
      company: Buffer.from(this.company).toString(),
      years_employed: this.years_employed,
    };
  }
}

const WorkInfoSchema = new Map([
  [
    WorkInfo,
    {
      kind: 'struct',
      fields: [
        ['name', [48]], // Fixed array of 48 bytes
        ['position', [48]], // Fixed array of 48 bytes
        ['company', [48]], // Fixed array of 48 bytes
        ['years_employed', 'u8'],
      ],
    },
  ],
]);
