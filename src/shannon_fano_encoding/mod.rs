//! # Shannon-Fano encoding
//!
//!

use std::rc::Rc;
pub mod node_tree;
use crate::utils::byte_array_ops::{ary_to_bits, to_n_bits};
use node_tree::*;

/// encode a byte array
pub fn encode(src: &[u8]) -> Vec<bool> {
    // create a table of frequency of each value
    let mut node_table: Vec<Node> = Vec::new();
    for ii in 0..MAX_CHAR {
        node_table.push(Node::new_with_code(ii as u8));
    }
    for v in src.to_vec() {
        node_table[v as usize].count += 1;
    }

    // sort the frequency table on the descending order
    node_table.sort();

    // create a code tree
    let mut total: u32 = 0;
    let mut x: u32 = 0;
    for n_ in node_table.iter() {
        if n_.count == 0 {
            break;
        }
        total += n_.count;
        x += 1;
    }
    // println!("{}, {}", total, x);
    let mut node_table: Vec<Rc<Node>> = node_table.into_iter().map(|x| Rc::new(x)).collect();
    let mut root: Rc<Node> = Rc::new(Node::new());
    if x < 2 {
        let left = node_table.remove(0);
        root.children.borrow_mut().push(Rc::clone(&left));
        *left.parent.borrow_mut() = Rc::downgrade(&root);
        let right = node_table.remove(0);
        root.children.borrow_mut().push(Rc::clone(&right));
        *right.parent.borrow_mut() = Rc::downgrade(&root);
    } else {
        root = make_tree(&node_table, 0, x - 1, total);
    }
    println!("before encode {:?}", root);

    // create a code table
    let mut code_table: Vec<(u8, u8)> = vec![(0, 0); MAX_CHAR];
    code_table = make_code(code_table, &root, 0, 0);
    // println!("{:?}, {}", code_table, code_table.len());

    // output the code tree
    let mut dst = tree_to_bits(&root);

    // output the code table
    for v in src.to_vec() {
        dst.append(&mut to_n_bits(
            code_table[v as usize].1,
            code_table[v as usize].0,
        ));
    }

    // padding for output in the bytes unit
    // let q: u8 = (dst.len() % 8) as u8;
    // if q > 0 {
    //     dst.append(&mut to_n_bits(q, 8u8 - q));
    // }
    dst
}

/// decode a byte array
pub fn decode(src: Vec<bool>) -> (Vec<u8>, Vec<bool>) {
    // read a code tree
    let (root, src): (Rc<Node>, Vec<bool>) = bits_to_tree(src);
    println!("after decode {:?}", root);

    // decode the encoded bit array
    let dst: Vec<u8> = Vec::new();

    (dst, src)
}
