import { Buffer } from 'node:buffer';
import * as borsh from 'borsh';

export class WorkInfo {
  name: string;
  position: string;
  company: string;
  years_employed: number;

  constructor(props: {
    name: string;
    position: string;
    company: string;
    years_employed: number;
  }) {
    this.name = props.name;
    this.position = props.position;
    this.company = props.company;
    this.years_employed = props.years_employed;
  }

  toBase58() {
    return borsh.serialize(WorkInfoSchema, this).toString();
  }

  toBuffer() {
    return Buffer.from(borsh.serialize(WorkInfoSchema, this));
  }

  static fromBuffer(buffer: Buffer) {
    return borsh.deserialize(WorkInfoSchema, WorkInfo, buffer);
  }
}

export const WorkInfoSchema = new Map([
  [
    WorkInfo,
    {
      kind: 'struct',
      fields: [
        ['name', 'string'],
        ['position', 'string'],
        ['company', 'string'],
        ['years_employed', 'u8'],
      ],
    },
  ],
]);
