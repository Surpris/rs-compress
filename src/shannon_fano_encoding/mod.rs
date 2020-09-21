//! # Shannon-Fano encoding
//!
//!

use std::rc::Rc;

pub mod node_tree;
use node_tree::*;

pub fn encode(src: &[u8]) -> Rc<Node> {
    let mut node_table: Vec<Node> = Vec::new();
    for ii in 0..MAX_CHAR {
        node_table.push(Node::new_with_code(ii as u8));
    }
    for v in src.to_vec() {
        node_table[v as usize].count += 1;
    }
    node_table.sort();
    let mut total: u32 = 0;
    let mut x: u32 = 0;
    for n_ in node_table.iter() {
        if n_.count == 0 {
            break;
        }
        total += n_.count;
        x += 1;
    }
    let mut node_table: Vec<Rc<Node>> = node_table.into_iter().map(|x| Rc::new(x)).collect();
    let mut root: Rc<Node> = Rc::new(Node::new());
    if x < 2 {
        root.children.borrow_mut().push(node_table.remove(0));
        root.children.borrow_mut().push(node_table.remove(0));
    } else {
        root = make_tree(&node_table, 0, x - 1, total);
    }
    root
}

pub fn decode() {}