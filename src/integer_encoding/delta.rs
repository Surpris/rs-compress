//! delta
//! 
//! delta encoding

use bit_vec::BitVec;
use super::alpha;

/// encode an integer
pub fn encode(v: u8) -> BitVec {
    let mut dst: BitVec = BitVec::new();
    let mut n_bits: u8 = 0;
    let mut n_plus_one: u16 = (v as u16 + 1) >> 1;
    while n_plus_one > 0 {
        n_bits += 1;
        n_plus_one >>= 1;
    }
    let n_bits_alpha: BitVec = alpha::encode(n_bits);
    
    // dst
    alpha::encode(v)
}

/// decode an encoded integer
pub fn decode(bytes: &BitVec) -> u8 {
    assert!(bytes.len() - 1 <= u8::MAX as usize);
    let mut dst: u8 = 0u8;
    for b_ in bytes.iter() {
        if b_ == true { break; }
        dst += 1;
    }
    dst
}
