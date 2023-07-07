
@program_id("F1ipperKF9EfD821ZbbYjS319LXYiBmjhzkkf5a26rC")
contract processing_instructions {

    @payer(payer) // payer for the data account, required by Solang but not used in this example
    constructor(address payer) {}

    function goToPark(string name, uint32 height) public pure {
        // Print messages to the program logs
        print("Welcome to the park, {:}".format(name));

        if (height >5) {
            print("You are tall enough to ride this ride. Congratulations.");
        } else {
            print("You are NOT tall enough to ride this ride. Sorry mate.");
        }
    }
}
