//! rs-compress

extern crate rs_compress;
use rs_compress as rsc;


// use rand::prelude::*;

fn main() {
    let a: Vec<u8> = vec![1, 2, 3, 4, 5];
    for v in a {
        println!("Original: {}", v);
        let encoded: Vec<bool> = rsc::integer_encoding::alpha::encode(v);
        println!("encoded to {:?}", encoded);
        let decoded: u8 = rsc::integer_encoding::alpha::decode(&encoded);
        println!("decoded to {}", decoded);
    }
}
