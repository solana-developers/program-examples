pub struct EnhancedAddressInfoExtender {
    pub state: [u8; 8],
    pub zip: u32,
}

pub struct EnhancedAddressInfo {
    pub name: [u8; 8],
    pub house_number: u8,
    pub street: [u8; 8],
    pub city: [u8; 8],
    pub state: [u8; 8],
    pub zip: u32,
}

impl EnhancedAddressInfo {
    pub const LEN: usize = 37;
}
