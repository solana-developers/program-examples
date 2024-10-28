import {
    Pubkey,
    type Result,
    Account,
    PoseidonError,
    SystemProgram,
} from "@solanaturbine/poseidon";

export default class Poseidon {
    static PROGRAM_ID = new Pubkey("GpVcgWdgVErgLqsn8VYUch6EqDerMgNqoLSmGyKrd6MR");
    static readonly MAX_NAME_LENGTH = 50;
    static readonly MAX_STREET_LENGTH = 50;
    static readonly MAX_CITY_LENGTH = 50;
    static readonly ACCOUNT_DISCRIMINATOR_SIZE = 8;

    static readonly INIT_SPACE =
        Poseidon.ACCOUNT_DISCRIMINATOR_SIZE +
        4 + Poseidon.MAX_NAME_LENGTH +
        1 +
        4 + Poseidon.MAX_STREET_LENGTH +
        4 + Poseidon.MAX_CITY_LENGTH;

    createAddressInfo(
        payer: Pubkey,
        state: AddressInfo,
        name: string,
        houseNumber: number,
        street: string,
        city: string
    ): Result {
        if (name.length > Poseidon.MAX_NAME_LENGTH) {
            return new PoseidonError("Name exceeds maximum length")
        }
        if (street.length > Poseidon.MAX_STREET_LENGTH) {
            return new PoseidonError("Street exceeds maximum length");
        }
        if (city.length > Poseidon.MAX_CITY_LENGTH) {
            return new PoseidonError("City exceeds maximum length");
        }

        if (houseNumber < 0 || houseNumber > 255) {
            return new PoseidonError("House number must be between 0 and 255");
        }

        state.name = name;
        state.houseNumber = houseNumber;
        state.street = street;
        state.city = city;
    }
}

export interface AddressInfo extends Account {
    name: string;
    houseNumber: number;
    street: string;
    city: string;
}