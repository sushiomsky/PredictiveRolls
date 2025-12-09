use burn::prelude::*;

/// Converts a hexadecimal string to a vector of binary values.
///
/// Each hex character is converted to 4 bits, represented as individual elements.
/// For example, 'F' becomes [1, 1, 1, 1] and '0' becomes [0, 0, 0, 0].
///
/// # Arguments
///
/// * `hex_str` - A string slice containing hexadecimal characters
///
/// # Returns
///
/// A vector of backend-specific float elements representing the binary values
pub fn hex_string_to_binary_vec<B: Backend>(hex_str: &str) -> Vec<B::FloatElem> {
    hex_str
        .chars()
        .flat_map(|chr| {
            let value = chr.to_digit(16).unwrap_or(0);
            (0..4)
                .rev()
                .map(move |i| ((value >> i) & 1).elem::<B::FloatElem>())
        })
        .collect()
}
