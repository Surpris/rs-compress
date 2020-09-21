//! rs-compress

extern crate rs_compress;
use rs_compress as rsc;
use rsc::shannon_fano_encoding::node_tree::*;
// use rsc::utils::*;

// use bit_vec::BitVec;
// use rand::prelude::*;
use std::rc::Rc;

fn main() {
    let src: Vec<u8> = vec![0, 0, 2, 1, 0, 4, 3, 2, 1, 1];
    let root: Rc<Node> = rsc::shannon_fano_encoding::encode(&src);
    println!("{:?}", root);
    // let node = make_tree(&node_table, 0, , high: u32, total: u32)
    // make_code(code_table, node, 0, 0);
    // for v in 0..255 {
    //     let b: Vec<bool> = rsc::integer_encoding::delta::encode(v);
    //     println!("{}, {}", v, to_string(&b));
    //     let (c, dst): (u8, Vec<bool>) = rsc::integer_encoding::delta::decode(b);
    //     println!("{}, {}", c, to_string(&dst));
    // }
    // let v = 255u8;
    // let b: Vec<bool> = rsc::integer_encoding::delta::encode(v);
    // println!("{}, {}", v, to_string(&b));
    // let (c, dst): (u8, Vec<bool>) = rsc::integer_encoding::delta::decode(b);
    // println!("{}, {}", c, to_string(&dst));
}
