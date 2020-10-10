//! # Huffman encoding
//!
//!

use std::collections::HashMap;
pub mod tree;
use crate::utils::bit_value_ops;
// use crate::utils::byte_value_ops;
use crate::utils::u64_value_ops;
use tree::*;

pub const NBR_OF_BITS: u64 = 8;

/// encode a byte array
pub fn encode(src: &[u8]) -> Vec<bool> {
    // create a table of frequency of each value
    let mut freq_table: Vec<u64> = vec![0u64; NBR_OF_CHARS];
    for v in src.to_vec() {
        freq_table[v as usize] += 1;
    }
    println!("{}", src.len());

    // create a code tree
    let tree: Tree = make_tree(&freq_table);
    print_tree(&tree, &mut vec![]);

    // create a code table
    let code_table: HashMap<u64, (u64, u64)> = make_code(&tree);
    println!("{:?}", code_table);

    // output the size of the input (bytes)
    let mut dst = u64_value_ops::to_bits(src.len() as u64);

    // output the code tree
    dst.append(&mut tree_to_bits(&tree, NBR_OF_BITS));

    // output the code table
    for v in src.to_vec() {
        let &(code, n) = code_table.get(&(v as u64)).unwrap();
        println!("{}, {}, {}", v, code, n);
        dst.append(&mut u64_value_ops::to_n_bits(code, n));
    }

    // padding for output in the bytes unit
    // let q: u8 = (dst.len() % 8) as u8;
    // if q > 0 {
    //     dst.append(&mut to_n_bits(q, 8u8 - q));
    // }
    dst
}

/// decode a byte array
pub fn decode(mut src: Vec<bool>) -> (Vec<u8>, Vec<bool>) {
    // read the size of the original byte array
    let mut buff: Vec<bool> = Vec::new();
    for _ in 0..64 {
        buff.push(src.remove(0));
    }
    println!("{:?}", buff);
    let mut size = bit_value_ops::to_u64(&buff);
    println!("{}", size);

    // read a code tree
    let (root, mut src) = bits_to_tree(src, NBR_OF_BITS);
    // println!("after decode {:?}", root);
    print_tree(&root, &mut vec![]);

    // create a code table
    // let mut _code_table: Vec<(u32, u8)> = vec![(0, 0); MAX_CHAR];
    // _code_table = make_code(_code_table, &root, 0, 0);

    // decode the encoded bit array
    let mut dst: Vec<u8> = Vec::new();
    while size > 0 {
        let (code, buff) = decode_sub(&root, src);
        dst.push(code as u8);
        src = buff;
        size -= 1;
    }
    (dst, src)
}

fn decode_sub(tree: &Tree, mut src: Vec<bool>) -> (u64, Vec<bool>) {
    let mut node = tree;
    loop {
        match &node {
            &Tree::Leaf(_, c) => return (*c, src),
            &Tree::Node(_, left, right) => {
                node = if src.remove(0) == false { &left } else { &right };
            }
        } 
    }
}