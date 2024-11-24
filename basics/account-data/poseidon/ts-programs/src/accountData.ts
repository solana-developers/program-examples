import { Account, Pubkey, Result, u8 } from '@solanaturbine/poseidon';

export default class AccountData {
  static PROGRAM_ID = new Pubkey('3cvZMR8oDVXVcxcfuPmBpsEWnGMYh2uomwYohNSJSWwk');

  createAddressInfo(addressInfo: AddressInfo, name: string, houseNumber: u8, street: string, city: string): Result {
    addressInfo.name = name;
    addressInfo.houseNumber = houseNumber;
    addressInfo.street = street;
    addressInfo.city = city;
  }
}

export interface AddressInfo extends Account {
  name: string;
  houseNumber: u8;
  street: string;
  city: string;
}
