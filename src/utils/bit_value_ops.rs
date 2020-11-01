//! bit_value_ops
//!
//! functions for a bit (array)

use super::cast_t2u;
use num_traits::PrimInt;
use std::mem::size_of;

/// convert a bit array into an array of values with the type `T`
/// # Panic
/// if src.len() % size_of<T> != 0
pub fn bits_to_values<T>(src: &[bool]) -> Vec<T>
where
    T: PrimInt,
{
    let size: usize = 8 * size_of::<T>();
    assert_eq!(src.len() % size, 0);
    let n_bytes = src.len() / size;
    let mut dst: Vec<T> = Vec::new();
    for ii in 0..n_bytes {
        dst.push(bits_to_value(&src[(ii * 8)..(ii + 1) * 8]));
    }
    dst
}

/// convert a bit array into a value with the type `T`
pub fn bits_to_value<T>(src: &[bool]) -> T
where
    T: PrimInt,
{
    let size: usize = src.len();
    let mut value: u64 = 0u64;
    for jj in 0..src.len() {
        if src[jj] == true {
            value |= 1 << (size - jj - 1);
        }
    }
    cast_t2u(value)
}

/// convert an array of values with the type `T` into a bit array
pub fn values_to_bits<T>(src: &[T]) -> Vec<bool>
where
    T: PrimInt,
{
    let mut dst: Vec<bool> = Vec::new();
    for v in src {
        dst.append(&mut value_to_bits(*v));
    }
    dst
}

/// convert a value with the type `T` into a bit array
pub fn value_to_bits<T>(src: T) -> Vec<bool>
where
    T: PrimInt,
{
    value_to_n_bits(src, 8 * size_of::<T>())
}

/// convert a value with the type `T` into a bit array with the length of `length`
pub fn value_to_n_bits<T>(src: T, length: usize) -> Vec<bool>
where
    T: PrimInt,
{
    let mut dst: Vec<bool> = Vec::new();
    let v: u64 = cast_t2u(src);
    let mut p: u64 = 1 << (length - 1);
    while p > 0 {
        dst.push((p & v) != 0);
        p >>= 1;
    }
    dst
}

/// convert a bit array into a string
/// with binary expression
pub fn to_string(src: &[bool]) -> String {
    let mut dst = String::new();
    for b_ in src.to_vec() {
        if b_ == false {
            dst += "0";
        } else {
            dst += "1";
        }
    }
    dst
}
