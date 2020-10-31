//! utils
//!
//! utility functions

use num_traits::PrimInt;

pub mod bit_value_ops;
pub mod byte_value_ops;
pub mod u32_value_ops;
pub mod u64_value_ops;

pub fn cast_t2u<T, U>(x: T) -> U
where
    T: PrimInt,
    U: PrimInt,
{
    U::from(x).unwrap()
}

pub fn bit_length_of_value<T, U>(v: T) -> U
where
    T: PrimInt,
    U: PrimInt,
{
    let mut p: u64 = 0;
    let mut q: u64 = (cast_t2u::<T, u64>(v) + 1) >> 1;
    while q > 0 {
        p += 1;
        q >>= 1;
    }
    cast_t2u(p)
}
