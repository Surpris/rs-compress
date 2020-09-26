//! byte_value_ops
//!
//! functions for a byte (array)

/// convert a byte array into a bit array
pub fn ary_to_bits(src: &[u8]) -> Vec<bool> {
    let mut dst: Vec<bool> = Vec::new();
    for v in src {
        dst.append(&mut to_bits(*v));
    }
    dst
}

/// convert a byte into a bit array
pub fn to_bits(src: u8) -> Vec<bool> {
    to_n_bits(src, 8)
}

/// convert a byte into a bit array
/// with the length of length
pub fn to_n_bits(src: u8, length: u8) -> Vec<bool> {
    let mut dst: Vec<bool> = Vec::new();
    let mut p: u8 = 1 << (length - 1);
    while p > 0 {
        dst.push((p & src) != 0);
        p >>= 1;
    }
    dst
}
