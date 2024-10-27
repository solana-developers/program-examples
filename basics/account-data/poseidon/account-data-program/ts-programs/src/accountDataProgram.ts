import { Account, Pubkey, Result, Signer, u8, u16, u32 } from '@solanaturbine/poseidon';

export default class AccountDataProgram {
  // Following Poseidon example pattern of static PROGRAM_ID
  static PROGRAM_ID = new Pubkey('CWy5sbubCYKdQ2ANmFmeZVRqxPJjE5NJ7S4SQBWHnPyF');

  create_address_info(payer: Signer, state: AddressInfo, houseNumber: u8, street: u8, cityCode: u32, name: string): Result {
    // Use nit() for initialization
    state.derive(['address_info', payer.key]).init();

    // Store the account data
    state.name = name;
    state.houseNumber = houseNumber;
    state.street = street;
    state.cityCode = cityCode;
  }
}

export interface AddressInfo extends Account {
  name: string;
  houseNumber: u8;
  street: u8;
  cityCode: u32;
}
