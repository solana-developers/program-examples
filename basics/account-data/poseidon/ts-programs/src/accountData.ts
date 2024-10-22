import { Account, Pubkey, Result, Signer, u8, u16, u32 } from '@solanaturbine/poseidon';

export default class AddressInfo {
  static PROGRAM_ID = new Pubkey('5wF2itZNsDcf5s1SdcdJPdgBSTFAKjj6YbdicLFYi8vN');

  initialize(owner: Signer, state: AddressInfoState, houseNumber: u8, streetNumber: u16, zipCode: u32, countryCode: u16): Result {
    state.derive(['account_data', owner.key]).init();

    state.owner = owner.key;
    state.streetNumber = streetNumber;
    state.houseNumber = houseNumber;
    state.zipCode = zipCode;
    state.countryCode = countryCode;
    state.bump = state.getBump();
  }

  edit(owner: Signer, state: AddressInfoState, houseNumber: u8, streetNumber: u16, zipCode: u32, countryCode: u16) {
    // Derive PDAs with stored bumps
    state.deriveWithBump(['account_data', owner.key], state.bump);
    state.streetNumber = streetNumber;
    state.houseNumber = houseNumber;
    state.zipCode = zipCode;
    state.countryCode = countryCode;
  }
}

export interface AddressInfoState extends Account {
  owner: Pubkey;
  bump: u8; // For PDA
  houseNumber: u8;
  streetNumber: u16;
  zipCode: u32;
  countryCode: u16;
}
