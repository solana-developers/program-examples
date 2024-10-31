import { Account, Pubkey, Result, Signer, u8, u16, u32 } from '@solanaturbine/poseidon';

export default class AccountDataProgram {
  // Following Poseidon example pattern of static PROGRAM_ID
  static PROGRAM_ID = new Pubkey('JvF1QDhgab1ARhACPWTAZnUymthGGmn3NXCj8i6mjSQ');

  create_address_info(payer: Signer, state: AddressInfo, name: string, houseNumber: u8, street: string, city: string): Result {
    // Use nit() for initialization
    state.derive(['address_info', payer.key]).init(payer);
    // Store the account data
    state.name = name;
    state.houseNumber = houseNumber;
    state.street = street;
    state.city = city;
  }
}

export interface AddressInfo extends Account {
  name: string;
  houseNumber: u8;
  street: string;
  city: string;
}
