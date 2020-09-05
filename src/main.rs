//! rs-compress

extern crate rs_compress;
use rs_compress as rsc;


// use rand::prelude::*;

fn main() {
    // let a: Vec<u8> = vec![3, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 0, 0, 0, 3];
    let a: Vec<u8> = vec![3, 2, 3, 4, 1, 1, 254, 2, 2, 2, 2, 1, 1, 2, 3];
    let b = rsc::run_length_encoding::zle::encode(&a);
    let c = rsc::run_length_encoding::zle::decode(&b);
    println!("{:?}", b);
    println!("{:?}", c);
}
