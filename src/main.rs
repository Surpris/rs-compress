//! rs-compress

extern crate rs_compress;
use rs_compress as rsc;
use rsc::shannon_fano_encoding::node_tree::*;

// use bit_vec::BitVec;
// use rand::prelude::*;
use std::rc::Rc;

fn main() {
    let mut node_table: Vec<Node> = Vec::new();
    for ii in 0..5 {
        node_table.push(Node::new_with_code(ii as u8));
    }
    let src: Vec<u32> = vec![0, 0, 2, 1, 0, 4, 3, 2, 1, 1];
    for v in src {
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
    // println!("{:?}", node_table);
    let mut node_table: Vec<Rc<Node>> = node_table.into_iter().map(|x| Rc::new(x)).collect();
    let mut root: Rc<Node> = Rc::new(Node::new());
    if x < 2 {
        root.children.borrow_mut().push(node_table.remove(0));
        root.children.borrow_mut().push(node_table.remove(0));
    } else {
        root = make_tree(&node_table, 0, x - 1, total);
    }
    println!("{:?}", root);
    // let node = make_tree(&node_table, 0, , high: u32, total: u32)
    // make_code(code_table, node, 0, 0);
    // for v in 0..255 {
    //     let b: Vec<bool> = rsc::integer_encoding::delta::encode(v);
    //     println!("{}, {}", v, print_bool_vec(&b));
    //     let (c, dst): (u8, Vec<bool>) = rsc::integer_encoding::delta::decode(b);
    //     println!("{}, {}", c, print_bool_vec(&dst));
    // }
    // let v = 255u8;
    // let b: Vec<bool> = rsc::integer_encoding::delta::encode(v);
    // println!("{}, {}", v, print_bool_vec(&b));
    // let (c, dst): (u8, Vec<bool>) = rsc::integer_encoding::delta::decode(b);
    // println!("{}, {}", c, print_bool_vec(&dst));
}

fn print_bool_vec(src: &Vec<bool>) -> String {
    let mut dst = String::new();
    for b_ in src.iter() {
        if *b_ == false {
            dst += "0";
        } else {
            dst += "1";
        }
    }
    dst
}
