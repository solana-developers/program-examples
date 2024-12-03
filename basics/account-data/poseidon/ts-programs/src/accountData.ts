import { Account, String as PoseidonString, Pubkey, type Result, Signer, u8 } from '@solanaturbine/poseidon';

export default class AccountData {
  static PROGRAM_ID = new Pubkey('CFiSZ4w8WcG7U8Axq3Lx5zpbyLiMMBde6HGKtZvRCn6U');

  createAddressInfo(
    owner: Signer,
    addressInfo: AddressInfo,
    name: PoseidonString<50>,
    houseNumber: u8,
    street: PoseidonString<50>,
    city: PoseidonString<50>,
  ): Result {
    addressInfo.derive(['address_info', owner.key]).init(owner);
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
