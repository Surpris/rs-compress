//! rs-compress

extern crate rs_compress;
use rs_compress as rsc;


use bit_vec::BitVec;
// use rand::prelude::*;

fn main() {
    let a: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
    for v in a {
        println!("Original: {}", v);
        let encoded: BitVec = rsc::integer_encoding::alpha::encode(v);
        println!("encoded to {:?}", encoded);
        let decoded: u8 = rsc::integer_encoding::alpha::decode(&encoded);
        println!("decoded to {}", decoded);
    }
}
