pub struct WorkInfo {
    pub name: [u8; 8],
    pub position: [u8; 8],
    pub company: [u8; 8],
    pub years_employed: u8,
}

impl WorkInfo {
    pub const LEN: usize = 25;
}
