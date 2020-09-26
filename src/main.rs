//! rs-compress

extern crate rs_compress;
use rs_compress as rsc;
// use rsc::shannon_fano_encoding::node_tree::*;
use rsc::utils::bit_value_ops::to_string;
// use rsc::utils::byte_array_ops as byteops;

// use bit_vec::BitVec;
// use rand::prelude::*;
// use std::rc::Rc;

fn main() {
    let src: Vec<u8> = vec![3, 3, 3, 3, 1, 1, 1, 2, 2];
    println!("{:?}", src);
    let encoded: Vec<bool> = rsc::shannon_fano_encoding::encode(&src);
    println!("{}, {}", to_string(&encoded), encoded.len());
    let (decoded, residual): (Vec<u8>, Vec<bool>) = rsc::shannon_fano_encoding::decode(encoded);
    println!("{:?}, {}", decoded, to_string(&residual));
}
