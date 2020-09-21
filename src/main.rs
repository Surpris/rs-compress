//! rs-compress

extern crate rs_compress;
use rs_compress as rsc;
use rsc::shannon_fano_encoding::node_tree::*;
use rsc::utils::bit_array_ops as bitops;
// use rsc::utils::byte_array_ops as byteops;

// use bit_vec::BitVec;
// use rand::prelude::*;
use std::rc::Rc;

fn main() {
    let src: Vec<u8> = vec![3, 3, 3, 3, 1, 1, 1, 2, 2];
    let node = rsc::shannon_fano_encoding::encode(&src);
    println!("{}, {}", bitops::to_string(&node), node.len());
    // println!("{:?}", node);
}
