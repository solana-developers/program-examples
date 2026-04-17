import {
  Account,
  Pubkey,
  Result,
  Signer,
  Str,
  u8,
} from "@solanaturbine/poseidon";

export default class AccountDataProgram {
  static PROGRAM_ID = new Pubkey("11111111111111111111111111111111");

  createAddressInfo(
    owner: Signer,
    addressInfo: AddressInfo,
    name: Str<32>,
    houseNumber: u8,
    street: Str<32>,
    city: Str<32>
  ): Result {
    addressInfo.derive(["address_info", owner.key]).init(owner);

    addressInfo.name = name;
    addressInfo.houseNumber = houseNumber;
    addressInfo.street = street;
    addressInfo.city = city;
  }
}

export interface AddressInfo extends Account {
  name: Str<32>;
  houseNumber: u8;
  street: Str<32>;
  city: Str<32>;
}
