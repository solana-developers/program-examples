import { Account, Pubkey, Result, Signer, u8, u16, u32 } from '@solanaturbine/poseidon';

/**
 * State account interface for storing address information
 */
export interface AddressInfoState extends Account {
  owner: Pubkey;
  bump: u8;
  houseNumber: u8;
  streetNumber: u16;
  zipCode: u32;
  countryCode: u16;
}

/**
 * Program class for managing address information
 */
export default class AddressInfoProgram {
  // Following Poseidon example pattern of static PROGRAM_ID
  static PROGRAM_ID = new Pubkey('5wF2itZNsDcf5s1SdcdJPdgBSTFAKjj6YbdicLFYi8vN');

  initialize(owner: Signer, state: AddressInfoState, houseNumber: u8, streetNumber: u16, zipCode: u32, countryCode: u16): Result {
    // Use derive() for PDA creation and init() for initialization
    state.derive(['address_info', owner.key]).init();

    // Store the account data
    state.owner = owner.key;
    state.houseNumber = houseNumber;
    state.streetNumber = streetNumber;
    state.zipCode = zipCode;
    state.countryCode = countryCode;
    state.bump = state.getBump();
  }

  edit(owner: Signer, state: AddressInfoState, houseNumber: u8, streetNumber: u16, zipCode: u32, countryCode: u16): Result {
    // Use deriveWithBump for existing PDAs
    state
      .derive(['address_info', owner.key]) // Derive PDA
      .has([owner]); // Validate owner using has() constraint

    // Update state data
    state.houseNumber = houseNumber;
    state.streetNumber = streetNumber;
    state.zipCode = zipCode;
    state.countryCode = countryCode;
  }
}
