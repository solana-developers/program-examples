
@program_id("F1ipperKF9EfD821ZbbYjS319LXYiBmjhzkkf5a26rC")
contract account_data {
    // A private instance of the AddressInfo struct
    // This is the data that is stored in the account
    AddressInfo private addressInfo;

    // The AddressInfo struct definition
    struct AddressInfo {
        string name;
        uint8 houseNumber;
        string street;
        string city;
    }

    @payer(payer) // "payer" is the account that pays to create the dataAccount
    constructor(
        @space uint16 space, // "space" allocated to the account (maximum 10240 bytes, maximum space that can be reallocate when creating account in program via a CPI) 
        string _name, 
        uint8 _houseNumber, 
        string _street, 
        string _city
    ) {
        // The AddressInfo instance is initialized with the data passed to the constructor
        addressInfo = AddressInfo(_name, _houseNumber, _street, _city);
    }

    // A function to get the addressInfo data stored on the account
    function get() public view returns (AddressInfo) {
        return addressInfo;
    }

    // A function to get the size in bytes of the stored AddressInfo
    function getAddressInfoSize() public view returns(uint) {
        uint size = 0;
        size += bytes(addressInfo.name).length;
        size += 1; // For houseNumber, which is uint8 and takes 1 byte
        size += bytes(addressInfo.street).length;
        size += bytes(addressInfo.city).length;
        return size;
    }
}
