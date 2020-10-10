//! tree
//!
//!

use std::cmp::Ordering;
use std::boxed::Box;
use std::collections::{BinaryHeap, HashMap};

use crate::utils::bit_value_ops::to_u64;
use crate::utils::u64_value_ops::to_n_bits;

pub const NBR_OF_CHARS: usize = 256;

/// Tree enum
// #[derive(Copy)]
pub enum Tree {
    Leaf(u64, u64),
    Node(u64, Box<Tree>, Box<Tree>),
}

use Tree::*;

impl Tree {
    fn get_count(&self) -> u64 {
        match self {
            &Leaf(n, _) => n,
            &Node(n, _, _) => n
        }
    }
    fn new_node() -> Tree {
        Node(0, Box::new(Leaf(0, 0)), Box::new(Leaf(0, 0)))
    }
    fn new_leaf() -> Tree {
        Leaf(0, 0)
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_count().cmp(&other.get_count()).reverse()
    }
}

impl Eq for Tree {}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.get_count() == other.get_count()
    }
}

/// make a code tree
pub fn make_tree(freq_table: &Vec<u64>) -> Tree {
    let mut que = BinaryHeap::new();
    for x in 0 .. freq_table.len() {
        let cnt = freq_table[x];
        if cnt > 0 { que.push(Leaf(cnt, x as u64)); }
    }
    if que.len() == 1 {
        if let Some(&Leaf(_, x)) = que.peek() {
            let c = if x == 0 { 1 } else { 0 };
            que.push(Leaf(0, c));
        }
    }
    loop {
        let n = que.pop().unwrap();
        if que.is_empty() { return n; }
        let m = que.pop().unwrap();
        let node = Node(n.get_count() + m.get_count(), Box::new(n), Box::new(m));
        que.push(node);
    }
}

/// print a Huffman tree
pub fn print_tree(node: &Tree, code: &mut Vec<u32>) {
    match node {
        &Leaf(_, c) => {
            print_space(code.len());
            println!("* {}, {:?}", c, code);
        },
        &Node(_, ref left, ref right) => {
            code.push(0);
            print_tree(left, code);
            code.pop();
            print_space(code.len());
            println!("*");
            code.push(1);
            print_tree(right, code);
            code.pop();
        }
    }
}
fn print_space(mut n: usize) {
    while n > 0 {
        print!("  ");
        n -= 1;
    }
}

/// make a code table
pub fn make_code(tree: &Tree) -> HashMap<u64, (u64, u64)> {
    let mut table = HashMap::new();
    make_code_sub(tree, 0, 0, &mut table);
    table
}

/// sub routine of make_code
/// tree : a code tree
/// n : node index (= nbr of bits)
/// code : code
/// table : a code table (value, (code, nbr of bits))
fn make_code_sub(tree: &Tree, n: u64, code: u64, table: &mut HashMap<u64, (u64, u64)>) {
    match &tree {
        &Leaf(_, c) => {
            table.insert(*c, (code, n));
        }
        &Node(_, left, right) => {
            make_code_sub(&left, n + 1, code << 1, table);
            make_code_sub(&right, n + 1, (code << 1) | 1, table);
        }
    }
}

/// convert a code tree into a bit array
pub fn tree_to_bits(tree: &Tree, n_bits: u64) -> Vec<bool> {
    let mut dst: Vec<bool> = Vec::new();
    match &tree {
        &Leaf(_, c) => {
            dst.push(true);
            dst.append(&mut to_n_bits(*c, n_bits))
        },
        &Node(_, left, right) => {
            dst.push(false);
            dst.append(&mut tree_to_bits(&left, n_bits));
            dst.append(&mut tree_to_bits(&right, n_bits));
        }
    }
    dst
}

/// convert a part of a bit array into a code tree
pub fn bits_to_tree(mut bits: Vec<bool>, n_bits: u64) -> (Tree, Vec<bool>) {
    let bit_: bool = bits.remove(0);
    if bit_ == true {
        let mut code_bits: Vec<bool> = Vec::new();
        for _ in 0..n_bits {
            code_bits.push(bits.remove(0));
        }
        (Leaf(0, to_u64(&code_bits)), bits)
    } else {
        let (left, bits): (Tree, Vec<bool>) = bits_to_tree(bits, n_bits);
        let (right, bits): (Tree, Vec<bool>) = bits_to_tree(bits, n_bits);
        (Node(0, Box::new(left), Box::new(right)), bits)
    }
}