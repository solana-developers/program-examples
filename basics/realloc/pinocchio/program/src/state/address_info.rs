pub struct AddressInfo {
    pub name: [u8; 8],
    pub house_number: u8,
    pub street: [u8; 8],
    pub city: [u8; 8],
}

impl AddressInfo {
    pub const LEN: usize = 25;
}
