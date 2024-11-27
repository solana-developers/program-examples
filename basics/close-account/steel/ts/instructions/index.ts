export * from './create';
export * from './close';

import * as borsh from '@coral-xyz/borsh';

export enum MyInstruction {
  CreateUser = 0,
  CloseUser = 1,
}

export const closeAccountSchema = borsh.rustEnum([borsh.struct([borsh.array(borsh.u8(), 64, 'name')], 'CreateUser'), borsh.struct([], 'CloseUser')]);
