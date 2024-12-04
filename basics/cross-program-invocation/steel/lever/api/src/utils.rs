pub fn str_to_bytes(name: &str) -> [u8; 32] {
    let mut name_bytes = [0u8; 32];
    name_bytes[..name.len()].copy_from_slice(name.as_bytes());
    name_bytes
}

pub fn bytes_to_str(bytes: &[u8; 32]) -> String {
    // Find the first occurrence of 0 (null terminator) or take all bytes if no null found
    let length = bytes.iter().position(|&x| x == 0).unwrap_or(bytes.len());

    // Convert the slice up to the null terminator (or full length) to a string
    String::from_utf8_lossy(&bytes[..length]).into_owned()
}
