import { Account, Pubkey, Result, u8 } from "@solanaturbine/poseidon";

export default class AccountData {
	static PROGRAM_ID = new Pubkey(
		"3cvZMR8oDVXVcxcfuPmBpsEWnGMYh2uomwYohNSJSWwk"
	);

	createAddressInfo(
		addressInfo: AddressInfo,
		name: Uint8Array,
		houseNumber: u8,
		// street: String,
		// city: String
	): Result {
		// addressInfo.name = name;
		addressInfo.houseNumber = houseNumber;
		// addressInfo.street = street;
		// addressInfo.city = city;
	}
}

export interface AddressInfo extends Account {
	name: Uint8Array;
	houseNumber: u8;
	street: String;
	city: String;
}
