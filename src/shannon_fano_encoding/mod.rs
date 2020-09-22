//! # Shannon-Fano encoding
//!
//!

use std::rc::Rc;
pub mod node_tree;
use crate::utils::byte_array_ops::to_n_bits;
use node_tree::*;

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
    println!("{:?}", root);

    // create a code table
    let mut code_table: Vec<(u8, u8)> = vec![(0, 0); MAX_CHAR];
    code_table = make_code(code_table, &root, 0, 0);
    println!("{:?}, {}", code_table, code_table.len());

    // output the code tree
    let mut dst = write_tree(&root);

    // output the code table
    for v in src.to_vec() {
        println!(
            "{}, {}, {}",
            v, code_table[v as usize].0, code_table[v as usize].1
        );
        dst.append(&mut to_n_bits(
            code_table[v as usize].1,
            code_table[v as usize].0,
        ));
    }
    dst
}

pub fn decode() {}
