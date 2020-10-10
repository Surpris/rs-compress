//! u32_value_ops
//!
//! functions for a usize (array)

use std::mem::size_of;

/// convert a byte array into a bit array
pub fn ary_to_bits(src: &[u32]) -> Vec<bool> {
    let mut dst: Vec<bool> = Vec::new();
    for v in src {
        dst.append(&mut to_bits(*v));
    }
    dst
}

/// convert a byte into a bit array
pub fn to_bits(src: u32) -> Vec<bool> {
    to_n_bits(src, 8 * size_of::<u32>() as u32)
}

/// convert a byte into a bit array
/// with the length of length
pub fn to_n_bits(src: u32, length: u32) -> Vec<bool> {
    let mut dst: Vec<bool> = Vec::new();
    let mut p: u32 = 1 << (length - 1);
    while p > 0 {
        dst.push((p & src) != 0);
        p >>= 1;
    }
    dst
}
