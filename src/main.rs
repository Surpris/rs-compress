//! rs-compress

extern crate rs_compress;
use rs_compress as rsc;


use bit_vec::BitVec;
// use rand::prelude::*;

fn main() {
    for v in 0..255 {
        let b: BitVec = rsc::integer_encoding::gamma::encode(v);
        let c: u8 = rsc::integer_encoding::gamma::decode(&b);
        println!("{}, {:?}, {}", v, b, c);
    }
    let v = 255u8;
    let b: BitVec = rsc::integer_encoding::gamma::encode(v);
    let c: u8 = rsc::integer_encoding::gamma::decode(&b);
    println!("{}, {:?}, {}", v, b, c);
}
