import { Account, String as PString, Pubkey, type Result, Signer, u8 } from '@solanaturbine/poseidon';

export default class AccountData {
  static PROGRAM_ID = new Pubkey('3PUaDfRezKNY9u2ffsAwgApxM3QYjztfYYcyNcuRKWmk');

  // Create Address Info instruction
  createAddressInfo(
    // ACCOUNTS

    payer: Signer,
    address_info: AddressInfoState,
    name: PString<25>,
    house_number: u8,
    street: PString<25>,
    city: PString<25>,
  ): Result {
    // CONTEXT

    // .derive([<seeds>]) ensures that <account> will be a PDA derived from the parameters as the seed
    // .init() ensures that the account will have the init constraint added to it.
    address_info.derive([payer.key]).init();

    // Store the data to the account
    address_info.name = name;
    address_info.house_number = house_number;
    address_info.street = street;
    address_info.city = city;
  }
}

export interface AddressInfoState extends Account {
  // String<MAX_LENGTH>; therefore, name, house_number, street and city are maximum 25 bytes each
  name: PString<25>;
  house_number: u8;
  street: PString<25>;
  city: PString<25>;
}
