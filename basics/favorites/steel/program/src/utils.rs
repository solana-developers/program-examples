use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ConversionError {
    StringTooLong(usize),
    StringTooShort(usize),
    VecLengthMismatch { expected: usize, actual: usize },
    InvalidUtf8,
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConversionError::StringTooLong(len) => {
                write!(f, "String length {} exceeds 32 bytes", len)
            }
            ConversionError::StringTooShort(len) => {
                write!(f, "String length {} is less than 32 bytes", len)
            }
            ConversionError::VecLengthMismatch { expected, actual } => {
                write!(
                    f,
                    "Vector length mismatch: expected {}, got {}",
                    expected, actual
                )
            }
            ConversionError::InvalidUtf8 => write!(f, "Invalid UTF-8 sequence in bytes"),
        }
    }
}

impl Error for ConversionError {}

// Convert string to bytes with padding
pub fn string_to_bytes32_padded(input: &str) -> Result<[u8; 32], ConversionError> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    if len > 32 {
        return Err(ConversionError::StringTooLong(len));
    }

    let mut result = [0u8; 32];
    result[..len].copy_from_slice(bytes);
    Ok(result)
}

// NEW: Convert bytes back to string, trimming trailing zeros
pub fn bytes32_to_string(bytes: &[u8; 32]) -> Result<String, ConversionError> {
    // Find the actual length by looking for the first zero or taking full length
    let actual_len = bytes.iter().position(|&b| b == 0).unwrap_or(32);

    // Convert the slice up to actual_len to a string
    String::from_utf8(bytes[..actual_len].to_vec()).map_err(|_| ConversionError::InvalidUtf8)
}

// Convert vec of strings to byte arrays with padding
pub fn strings_to_bytes32_array_padded<const N: usize>(
    inputs: Vec<&str>,
) -> Result<[[u8; 32]; N], ConversionError> {
    if inputs.len() != N {
        return Err(ConversionError::VecLengthMismatch {
            expected: N,
            actual: inputs.len(),
        });
    }

    let mut result = [[0u8; 32]; N];
    for (i, input) in inputs.iter().enumerate() {
        result[i] = string_to_bytes32_padded(input)?;
    }
    Ok(result)
}

// NEW: Convert array of byte arrays back to vec of strings
pub fn bytes32_array_to_strings<const N: usize>(
    bytes_array: &[[u8; 32]; N],
) -> Result<Vec<String>, ConversionError> {
    let mut result = Vec::with_capacity(N);
    for bytes in bytes_array.iter() {
        result.push(bytes32_to_string(bytes)?);
    }
    Ok(result)
}

// // Convert string to bytes with padding
// pub fn string_to_bytes32_padded(input: &str) -> Result<[u8; 32], ConversionError> {
//     let bytes = input.as_bytes();
//     let len = bytes.len();

//     if len > 32 {
//         return Err(ConversionError::StringTooLong(len));
//     }

//     let mut result = [0u8; 32];
//     result[..len].copy_from_slice(bytes);
//     Ok(result)
// }

// // NEW: Convert bytes back to string, trimming trailing zeros
// pub fn bytes32_to_string(bytes: &[u8; 32]) -> Result<String, ConversionError> {
//     // Find the actual length by looking for the first zero or taking full length
//     let actual_len = bytes.iter().position(|&b| b == 0).unwrap_or(32);

//     // Convert the slice up to actual_len to a string
//     String::from_utf8(bytes[..actual_len].to_vec()).map_err(|_| ConversionError::InvalidUtf8)
// }

// // Convert vec of strings to byte arrays with padding
// pub fn strings_to_bytes32_array_padded<const N: usize>(
//     inputs: Vec<&str>,
// ) -> Result<[[u8; 32]; N], ConversionError> {
//     if inputs.len() != N {
//         return Err(ConversionError::VecLengthMismatch {
//             expected: N,
//             actual: inputs.len(),
//         });
//     }

//     let mut result = [[0u8; 32]; N];
//     for (i, input) in inputs.iter().enumerate() {
//         result[i] = string_to_bytes32_padded(input)?;
//     }
//     Ok(result)
// }

// // NEW: Convert array of byte arrays back to vec of strings
// pub fn bytes32_array_to_strings<const N: usize>(
//     bytes_array: &[[u8; 32]; N],
// ) -> Result<Vec<String>, ConversionError> {
//     let mut result = Vec::with_capacity(N);
//     for bytes in bytes_array.iter() {
//         result.push(bytes32_to_string(bytes)?);
//     }
//     Ok(result)
// }
