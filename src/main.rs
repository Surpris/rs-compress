//! rs-compress

extern crate rs_compress;
use rs_compress as rsc;
// use rsc::shannon_fano_encoding::node_tree::*;
use rsc::utils::bit_value_ops::to_string;
// use rsc::utils::byte_array_ops as byteops;

// use bit_vec::BitVec;
use rand::prelude::*;
// use std::rc::Rc;

fn main() {
    let code: Vec<u8> = vec![7, 6, 5, 4, 3, 2, 1, 0];
    let counts: Vec<u32> = vec![8, 8 ,4, 4, 2, 2, 1, 1];
    let mut src: Vec<u8> = Vec::new();
    for ii in 0..code.len() {
        for _ in 0..counts[ii] {
            src.push(code[ii]);
        }
    }
    let _encoded: Vec<bool> = rsc::shannon_fano_encoding::encode(&src);
}

#[allow(dead_code)]
fn test_shannon_fano() {
    let mut rng = rand::thread_rng();
    let mut a = [0u8; 256];
    rng.fill_bytes(&mut a);
    let mut a: Vec<u8> = Vec::new();
    for ii in 0..259u16 {
        a.push((ii % 256) as u8);
    }
    // let src: Vec<u8> = vec![3, 3, 3, 3, 1, 1, 1, 2, 2];
    // println!("{:?}", a.to_vec());
    let encoded: Vec<bool> = rsc::shannon_fano_encoding::encode(&a);
    // println!("{}, {}", to_string(&encoded), encoded.len());
    let (decoded, _residual): (Vec<u8>, Vec<bool>) = rsc::shannon_fano_encoding::decode(encoded);
    println!("{}", to_string(&_residual));
    // println!("{:?}, {}", decoded, to_string(&residual));
    println!("{}, {}", a.len(), decoded.len());
    // assert_eq!(a.to_vec(), decoded);
}