import { Account, Pubkey, Result, Signer, u8, u16, u32, String } from '@solanaturbine/poseidon';

export default class AccountDataProgram {
  // Following Poseidon example pattern of static PROGRAM_ID
  static PROGRAM_ID = new Pubkey('JvF1QDhgab1ARhACPWTAZnUymthGGmn3NXCj8i6mjSQ');

  create_address_info(payer: Signer, state: AddressInfo, name: String<50>, houseNumber: u8, street: String<50>, city: String<50>): Result {
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
  name: String<50>;
  houseNumber: u8;
  street: String<50>;
  city: String<50>;
}
