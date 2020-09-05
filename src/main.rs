//! rs-compress

extern crate rs_compress;
use rs_compress as rsc;


// use rand::prelude::*;

fn main() {
    let a: Vec<u8> = vec![3, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 4, 4, 4, 3];
    let b = rsc::run_length_encoding::switched_rle::encode(&a);
    let c = rsc::run_length_encoding::switched_rle::decode(&b);
    println!("{:?}", b);
    println!("{:?}", c);
}
