
@program_id("F1ipperKF9EfD821ZbbYjS319LXYiBmjhzkkf5a26rC")
contract counter {
    // The counter value that is stored in the account
    uint64 private count;

    // The constructor is used to create a new counter account
    @payer(payer)  // The "payer" pays for the counter account creation
    constructor(address payer) {
        // Initialize the count to zero
        count = 0;
    }

    // Increments the count by one.
    function increment() public {
        count += 1;
    }

    // Returns the count value
    function get() public view returns (uint64) {
        return count;
    }
}
