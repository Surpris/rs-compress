//! u64_value_ops
//!
//! functions for a usize (array)

use std::mem::size_of;

/// convert a u64-integer array into a bit array
pub fn ary_to_bits(src: &[u64]) -> Vec<bool> {
    let mut dst: Vec<bool> = Vec::new();
    for v in src {
        dst.append(&mut to_bits(*v));
    }
    dst
}

/// convert a u64 integer into a bit array
pub fn to_bits(src: u64) -> Vec<bool> {
    to_n_bits(src, 8 * size_of::<u64>() as u64)
}

/// convert a u64 integer into a bit array
/// with the length of length
pub fn to_n_bits(src: u64, length: u64) -> Vec<bool> {
    let mut dst: Vec<bool> = Vec::new();
    let mut p: u64 = 1 << (length - 1);
    while p > 0 {
        dst.push((p & src) != 0);
        p >>= 1;
    }
    dst
}
