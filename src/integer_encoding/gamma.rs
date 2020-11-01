//! gamma
//!
//! gamma encoding

use super::alpha;
use crate::utils::cast_t2u;
use num_traits::PrimInt;

/// encode an unsigned integer
pub fn encode<T>(v: T) -> Vec<bool>
where
    T: PrimInt,
{
    let mut n_bits: u64 = 0;
    let mut n_plus_one: u64 = (cast_t2u::<T, u64>(v) + 1) >> 1;
    while n_plus_one > 0 {
        n_bits += 1;
        n_plus_one >>= 1;
    }
    let mut dst: Vec<bool> = alpha::encode::<u64>(n_bits);
    if n_bits > 0 {
        let mut p: u64 = 1 << (n_bits - 1);
        while p > 0 {
            dst.push((p & (cast_t2u::<T, u64>(v) + 1)) != 0);
            p >>= 1;
        }
    }
    dst
}

/// decode an encoded unsigned integer
pub fn decode<T>(bits: Vec<bool>) -> (T, Vec<bool>)
where
    T: PrimInt,
{
    let (n_bits, mut src): (T, Vec<bool>) = alpha::decode::<T>(bits);
    let n_bits = cast_t2u::<T, u32>(n_bits);
    let mut value: u64 = 2u64.pow(n_bits);
    for ii in 0..(n_bits) {
        if src.remove(0) == true {
            value += 2u64.pow(n_bits - ii - 1);
        }
    }
    (cast_t2u::<u64, T>(value - 1), src)
}
