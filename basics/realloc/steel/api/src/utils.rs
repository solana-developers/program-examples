pub fn str_to_bytes(name: &str) -> [u8; MAX_STR_LEN] {
    let mut name_bytes = [0u8; MAX_STR_LEN];
    name_bytes[..name.len()].copy_from_slice(name.as_bytes());
    name_bytes
}
