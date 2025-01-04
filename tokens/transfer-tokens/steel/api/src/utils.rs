pub fn str_to_bytes<const N: usize>(str: &str) -> [u8; N] {
    let mut str_bytes = [0u8; N];
    let copy_len = str.len().min(N);
    str_bytes[..copy_len].copy_from_slice(&str.as_bytes()[..copy_len]);
    str_bytes
}
