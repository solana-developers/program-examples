import {
  Account,
  Pubkey,
  Signer,
  String,
  u8,
  type Result,
  u32,
} from "@solanaturbine/poseidon";

export default class AccountData {
  static PROGRAM_ID = new Pubkey(
    "3PUaDfRezKNY9u2ffsAwgApxM3QYjztfYYcyNcuRKWmk"
  );

  // Create Address Info instruction
  createAddressInfo(
    // ACCOUNTS

    payer: Signer,
    address_info: AddressInfoState,
    house_number: u8,
    street_number: u8,
    cityZipCode: u32,
    name: String<25>
  ): Result {
    // CONTEXT

    // .derive([<seeds>]) ensures that <account> will be a PDA derived from the parameters as the seed
    // .init() ensures that the account will have the init constraint added to it.
    address_info.derive([payer.key]).init();

    // Store the data to the account
    address_info.name = name;
    address_info.house_number = house_number;
    address_info.street_number = street_number;
    address_info.cityZipCode = cityZipCode;
  }
}

export interface AddressInfoState extends Account {
  house_number: u8;
  name: String<25>; // The name has a MAX_LENGTH of 25 bytes
  street_number: u8;
  cityZipCode: u32;
}
