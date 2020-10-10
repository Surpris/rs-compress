//! # Huffman encoding
//!
//!

use std::rc::Rc;
pub mod node_tree;
// pub mod tree;
use crate::utils::bit_value_ops;
// use crate::utils::byte_value_ops;
use crate::utils::u32_value_ops;
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
    println!("{}, {}", total, x);
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
    // println!("before encode {:?}", root);
    // print_tree(&root, 0);

    // create a code table
    let mut code_table: Vec<(u32, u8)> = vec![(0, 0); MAX_CHAR];
    code_table = make_code(code_table, &root, 0, 0);
    // println!("{:?}, {}", code_table, code_table.len());

    // output the size of the input (bytes)
    let mut dst = u32_value_ops::to_bits(src.len() as u32);
    // output the code tree
    dst.append(&mut tree_to_bits(&root));

    // output the code table
    for v in src.to_vec() {
        println!("{}, {:?}", v, code_table[v as usize]);
        dst.append(&mut u32_value_ops::to_n_bits(
            code_table[v as usize].1 as u32,
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
pub fn decode(mut src: Vec<bool>) -> (Vec<u8>, Vec<bool>) {
    // read the size of the original byte array
    let mut buff: Vec<bool> = Vec::new();
    for _ in 0..32 {
        buff.push(src.remove(0));
    }
    let mut size = bit_value_ops::to_u32(&buff);
    // println!("{}", size);

    // read a code tree
    let (root, mut src) = bits_to_tree(src);
    // println!("after decode {:?}", root);
    // print_tree(&root, 0);

    // create a code table
    let mut _code_table: Vec<(u32, u8)> = vec![(0, 0); MAX_CHAR];
    _code_table = make_code(_code_table, &root, 0, 0);

    // decode the encoded bit array
    let mut dst: Vec<u8> = Vec::new();
    while size > 0 {
        let (node, buff) = search_leaf(Rc::clone(&root), src);
        dst.push(node.code);
        size -= 1;
        src = buff;
    }
    (dst, src)
}
