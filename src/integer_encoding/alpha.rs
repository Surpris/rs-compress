//! alpha
//!
//! alpha encoding

use num_traits::PrimInt;
// use std::any::type_name;
use crate::utils::cast_t2u;

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

// /// encode an u8 integer
// pub fn encode_u8(v: u8) -> Vec<bool> {
//     let mut dst: Vec<bool> = Vec::new();
//     for _ in 0..v {
//         dst.push(false);
//     }
//     dst.push(true);
//     dst
// }

// /// encode an u16 integer
// pub fn encode_u16(v: u16) -> Vec<bool> {
//     let mut dst: Vec<bool> = Vec::new();
//     for _ in 0..v {
//         dst.push(false);
//     }
//     dst.push(true);
//     dst
// }

// /// encode an u32 integer
// pub fn encode_u32(v: u32) -> Vec<bool> {
//     let mut dst: Vec<bool> = Vec::new();
//     for _ in 0..v {
//         dst.push(false);
//     }
//     dst.push(true);
//     dst
// }

// /// encode an u64 integer
// pub fn encode_u64(v: u64) -> Vec<bool> {
//     let mut dst: Vec<bool> = Vec::new();
//     for _ in 0..v {
//         dst.push(false);
//     }
//     dst.push(true);
//     dst
// }

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

/// decode an encoded u8 integer
pub fn decode_u8(mut src: Vec<bool>) -> (u8, Vec<bool>) {
    let mut value = 0u8;
    while src.remove(0) == false {
        value += 1;
    }
    (value, src)
}

/// decode an encoded u16 integer
pub fn decode_u16(mut src: Vec<bool>) -> (u16, Vec<bool>) {
    let mut value = 0u16;
    while src.remove(0) == false {
        value += 1;
    }
    (value, src)
}

/// decode an encoded u32 integer
pub fn decode_u32(mut src: Vec<bool>) -> (u32, Vec<bool>) {
    let mut value = 0u32;
    while src.remove(0) == false {
        value += 1;
    }
    (value, src)
}

/// decode an encoded u64 integer
pub fn decode_u64(mut src: Vec<bool>) -> (u64, Vec<bool>) {
    let mut value = 0u64;
    while src.remove(0) == false {
        value += 1;
    }
    (value, src)
}
