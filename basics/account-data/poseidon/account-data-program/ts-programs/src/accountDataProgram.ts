import { Account, Pubkey, Result, Signer, Str, u8, u16, u32 } from '@solanaturbine/poseidon';

export default class AccountDataProgram {
  // Following Poseidon example pattern of static PROGRAM_ID
  static PROGRAM_ID = new Pubkey('JvF1QDhgab1ARhACPWTAZnUymthGGmn3NXCj8i6mjSQ');

  create_address_info(payer: Signer, state: AddressInfo, name: Str<25>, houseNumber: u8, street: Str<25>, city: Str<25>): Result {
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
  name: Str<25>;
  houseNumber: u8;
  street: Str<25>;
  city: Str<25>;
}
