//! alpha
//!
//! alpha encoding

use crate::utils::cast_t2u;
use num_traits::PrimInt;

/// encode an unsigned integer
pub fn encode<T>(v: T) -> Vec<bool>
where
    T: PrimInt,
{
    let mut dst: Vec<bool> = Vec::new();
    for _ in 0..cast_t2u::<T, u64>(v) {
        dst.push(false);
    }
    dst.push(true);
    dst
}

/// decode an encoded unsigned integer
pub fn decode<T>(mut src: Vec<bool>) -> (T, Vec<bool>)
where
    T: PrimInt + Copy,
{
    let mut value = 0u64;
    while src.remove(0) == false {
        value += 1;
    }
    (cast_t2u::<u64, T>(value), src)
}
