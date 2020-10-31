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
