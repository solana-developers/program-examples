import { Account, Pubkey, Result, Signer, SystemAccount, u8, u64 } from '@solanaturbine/poseidon';

export default class RentProgram {
  static PROGRAM_ID = new Pubkey('EHjrAJo1Ld77gkq6Pp2ErQHcC6FghT8BEPebNve8bAvj');

  //Create a new system account
  createSystemAccount(owner: Signer, account: AddressData, id: u64, zipCode: u64): Result {
    account.derive(['account']).init();

    account.accountBump = account.getBump();
    //Set owner of the account
    account.owner = owner.key;
    //Set id
    account.id = id;
    //Set zipCode
    account.zipCode = zipCode;
  }
}

export interface AddressData extends Account {
  owner: Pubkey;
  id: u64;
  zipCode: u64;
  accountBump: u8;
}
