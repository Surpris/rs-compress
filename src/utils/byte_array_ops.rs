//! byte_array_ops
//! 
//! functions for a byte array


pub fn ary_to_bits(src: &[u8]) -> Vec<bool> {
    let mut dst: Vec<bool> = Vec::new();
    for v in src {
        dst.append(&mut to_bits(*v));
    }
    dst
}

pub fn to_bits(src: u8) -> Vec<bool> {
    let mut dst: Vec<bool> = Vec::new();
    let mut p: u8 = 1 << 7;
    while p > 0 {
        dst.push((p & src) != 0);
        p >>= 1;
    }
    dst
}

pub fn put_bits() {}