//! gamma
//! 
//! gamma encoding

// use bit_vec::BitVec;
use super::alpha;

/// encode an integer
pub fn encode(v: u8) -> Vec<bool> {
    let mut n_bits: u8 = 0;
    let mut n_plus_one: u16 = (v as u16 + 1) >> 1;
    while n_plus_one > 0 {
        n_bits += 1;
        n_plus_one >>= 1;
    }
    let mut dst: Vec<bool> = alpha::encode(n_bits);
    if n_bits > 0 {
        let mut p: u8 = 1 << (n_bits - 1);
        while p > 0 {
            dst.push(((p as u16) & (v as u16 + 1)) != 0);
            p >>= 1;
        }
    }
    dst
}

/// decode an encoded integer
pub fn decode(bits: Vec<bool>) -> (u8, Vec<bool>) {
    let (n_bits, mut src): (u8, Vec<bool>) = alpha::decode(bits);
    let mut value: u32 = 2u32.pow(n_bits as u32);
    for ii in 0..(n_bits) {
        if src.remove(0) == true {
            value += 2u32.pow((n_bits - ii -1) as u32);
        }
    }
    ((value - 1) as u8, src)
}
