import { Account, String as PoseidonString, Pubkey, type Result, u8 } from '@solanaturbine/poseidon';

export default class AccountData {
  static PROGRAM_ID = new Pubkey('3cvZMR8oDVXVcxcfuPmBpsEWnGMYh2uomwYohNSJSWwk');

  createAddressInfo(
    addressInfo: AddressInfo,
    name: PoseidonString<50>,
    houseNumber: u8,
    street: PoseidonString<50>,
    city: PoseidonString<50>,
  ): Result {
    addressInfo.name = name;
    addressInfo.houseNumber = houseNumber;
    addressInfo.street = street;
    addressInfo.city = city;
  }
}

export interface AddressInfo extends Account {
  name: PoseidonString<50>;
  houseNumber: u8;
  street: PoseidonString<50>;
  city: PoseidonString<50>;
}
