//! delta
//! 
//! delta encoding

// use bit_vec::BitVec;
use super::gamma;

// encode an integer
// pub fn encode(v: u8) -> Vec<bool> {
//     let mut n_bits: u8 = 0;
//     let mut n_plus_one: u16 = (v as u16 + 1) >> 1;
//     while n_plus_one > 0 {
//         n_bits += 1;
//         n_plus_one >>= 1;
//     }
//     let mut dst: Vec<bool> = gamma::encode(n_bits);
//     if n_bits > 0 {
//         let mut p: u8 = 1 << (n_bits - 1);
//         while p > 0 {
//             dst.push(((p as u16) & (v as u16 + 1)) != 0);
//             p >>= 1;
//         }
//     }
//     dst
// }

// decode an encoded integer
// pub fn decode(mut bits: Vec<bool>) -> u8 {
//     let n_bits: u8 = gamma::decode(bits);
//     if n_bits > 0 {
//         let mut buff: Vec<bool> = Vec::new();
//         if n_bits < 8 {
//             for _ in 0..(8 - n_bits - 1) {
//                 buff.push(false);
//             }
//         }
//         buff.push(true);
//         // for ii in 0..n_bits {
//         //     let ind: usize = bits.len() - n_bits as usize + ii as usize;
//         //     buff.push(bits[ind]);
//         // }
//         // let bytes = buff.to_bytes();
//         let mut dst: u32 = 1;
//         // for ii in 0..bytes.len() {
//         //     let p: usize = bytes.len() - ii - 1;
//         //     dst += u32::pow(u8::MAX as u32 + 1, p as u32) * bytes[ii] as u32;
//         // }
//         (dst - 1) as u8
//     } else {
//         0u8
//     }
// }