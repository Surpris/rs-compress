//! bit_value_ops
//!
//! functions for a bit (array)

use super::cast_t2u;
use num_traits::PrimInt;
use std::mem::size_of;

/// convert a bit array into an array of values with the type `T`
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
    let size: usize = 8 * size_of::<T>();
    assert_eq!(src.len() % size, 0);
    let mut value: u64 = 0u64;
    for jj in 0..size {
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

/// ------- deprecated below ------- ///
/// convert a bit array into a byte arrays
pub fn to_bytes(src: &[bool]) -> Vec<u8> {
    assert_eq!(src.len() % 8, 0);
    let n_bytes = src.len() / 8;
    let mut dst: Vec<u8> = Vec::new();
    for ii in 0..n_bytes {
        dst.push(to_byte(&src[(ii * 8)..(ii + 1) * 8]));
    }
    dst
}

/// convert a bit array into a byte
pub fn to_byte(src: &[bool]) -> u8 {
    assert_eq!(src.len(), 8);
    let mut value: u8 = 0u8;
    for jj in 0..8 {
        if src[jj] == true {
            value += 2u8.pow((8 - jj - 1) as u32);
        }
    }
    value
}

/// convert a bit array into a usize value
pub fn to_u32(src: &[bool]) -> u32 {
    let size: usize = 8 * size_of::<u32>();
    assert_eq!(src.len(), size);
    let mut value: u32 = 0;
    for jj in 0..size {
        if src[jj] == true {
            value += 2u32.pow((size - jj - 1) as u32);
        }
    }
    value
}

/// convert a bit array into a usize value
pub fn to_u64(src: &[bool]) -> u64 {
    // let size: usize = 8 * size_of::<u64>();
    // assert_eq!(src.len(), size);
    let size: usize = src.len();
    let mut value: u64 = 0;
    for jj in 0..size {
        if src[jj] == true {
            value += 2u64.pow((size - jj - 1) as u32);
        }
    }
    value
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
