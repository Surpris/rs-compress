//! tree
//!
//!

use std::boxed::Box;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::ops::Bound::{Included, Unbounded};

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
            &Node(n, _, _) => n,
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
/// freq_table is sorted on the descending order.
pub fn make_tree(freq_table: &Vec<u64>) -> Tree {
    let mut leaf_map: BTreeMap<u64, Tree> = BTreeMap::new();
    let mut total: u64 = 0;
    for ii in 0..freq_table.len() {
        let count = freq_table[ii];
        total += count;
        if count > 0 {
            leaf_map.insert(total, Leaf(count, ii as u64));
        }
    }
    if leaf_map.len() == 1 {
        let keys: Vec<_> = leaf_map.keys().cloned().collect();
        if let Some((_, &Leaf(_, x))) = leaf_map.get_key_value(&keys[0]) {
            let c = if x == 0 { 1 } else { 0 };
            leaf_map.insert(total + 1, Leaf(0, c));
        }
    }
    let mut root: Tree = Tree::new_node();
    root
}

fn make_tree_step(leaf_map: &BTreeMap<u64, Tree>) -> Tree {
    let keys: Vec<_> = leaf_map.keys().cloned().collect();
    if keys.len() == 1 {
        if let Some((_, &Leaf(c, x))) = leaf_map.get_key_value(&keys[0]) {
            return Leaf(c, x);
        } else {
            panic!();
        }
    }
    let center: u64 = (keys[0] + keys[keys.len() - 1]) / 2;

    // TODO:
    // 1. check how to implement Copy trait
    // 2. go back to and modify the first implementation so that
    //    it uses u64 data type

    // let mut left_map: BTreeMap<u64, Tree> = BTreeMap::new();
    // for (&key, &value) in leaf_map.range((Unbounded, Included(&center))) {}

    // let left: Tree = make_tree_step(&left_map);
    Tree::new_node()
}
