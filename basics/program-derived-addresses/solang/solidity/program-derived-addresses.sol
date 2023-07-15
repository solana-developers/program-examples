
import "solana";

@program_id("F1ipperKF9EfD821ZbbYjS319LXYiBmjhzkkf5a26rC")
contract program_derived_addresses {
    // A private instance of the PageVisits struct
    // This is the data that is stored in the account
    PageVisits private accountData;

    // The PageVisits struct definition
    struct PageVisits {
        uint32 pageVisits;
        bytes1 bump;
    }

    // The constructor is used to create a new account
    // The seeds, bump, and programId are used to derive a unique and deterministic pda (program derived address) to use as the account address
    @payer(payer) // "payer" is the account that pays for creating the account
    @seed("page_visits") // hardcoded seed
    constructor(
        @seed bytes payer, // additional seed using the payer address
        @bump bytes1 bump // bump seed to derive the pda
    ) {
        // Independently derive the PDA address from the seeds, bump, and programId
        (address pda, bytes1 _bump) = try_find_program_address(["page_visits", payer], type(program_derived_addresses).program_id);

        // Verify that the bump passed to the constructor matches the bump derived from the seeds and programId
        // This ensures that only the canonical pda address can be used to create the account (first bump that generates a valid pda address)
        require(bump == _bump, 'INVALID_BUMP');

        // The PageVisits instance is initialized with pageVisits set to zero and bump set to the bump passed to the constructor
        accountData = PageVisits(0, bump);
    }

    // Increments the pageVisits by one.
    function incrementPageVisits() public {
        accountData.pageVisits += 1;
    }

    // Returns the accountData (pageVisits and bump) stored on the account
    function get() public view returns (PageVisits) {
        return accountData;
    }
}
