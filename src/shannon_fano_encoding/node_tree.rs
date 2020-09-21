//! node_tree
//!
//!

use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
// use std::boxed::Box;
use std::cmp::Ordering;

use crate::utils::byte_array_ops::to_bits;

pub const MAX_CHAR: usize = 256;

#[derive(Debug)]
pub struct Node {
    pub code: u8,
    pub count: u32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            code: 0,
            count: 0,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }

    pub fn new_with_code(code: u8) -> Node {
        Node {
            code,
            count: 0,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }

    // pub fn set_left(&self, node: Node) {
    //     let left = Rc::new(node);
    //     if self.children.borrow_mut().len() > 0 {
    //         self.children.borrow_mut()[0] = Rc::clone(&left);
    //     } else {
    //         self.children.borrow_mut().push(Rc::clone(&left));
    //     }
    //     *left.parent.borrow_mut() = Rc::downgrade(&Rc::new(*self));
    // }

    // pub fn set_right(&mut self, node: Node) {
    //     let right = Rc::new(node);
    //     if self.children.borrow_mut().len() == 0 {
    //         self.children.borrow_mut().push(Rc::new(Node::new()));
    //     } else if self.children.borrow_mut().len() == 1 {
    //         self.children.borrow_mut().push(Rc::clone(&right));
    //     } else {
    //         self.children.borrow_mut()[1] = Rc::clone(&right);
    //     }
    //     *right.parent.borrow_mut() = Rc::downgrade(&Rc::new(*self));
    // }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.count.cmp(&self.count)
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        other.count.partial_cmp(&self.count)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.count == other.count
    }
}

/// make a code tree
pub fn make_tree(node_table: &Vec<Rc<Node>>, low: u32, high: u32, total: u32) -> Rc<Node> {
    
    assert!(node_table.len() <= MAX_CHAR);
    if low >= high {
        let node = node_table[high as usize].clone();
        // println!("make tree from {}", node.code);
        node
    } else {
        let half: u32 = total / 2;
        let mut c: u32 = 0;
        let mut ind: u32 = 0;
        for ii in low..(high + 1) {
            let p = c;
            c += node_table[ii as usize].count;
            ind = ii;
            if c >= half {
                if 2 * half < c + p {
                    c = p;
                    ind -= 1;
                }
                break;
            }
        }
        // println!("{}, {}", c, ind);
        let node = Rc::new(Node::new());

        // left side
        let left = make_tree(&node_table, low, ind, c);
        node.children.borrow_mut().push(Rc::clone(&left));
        *left.parent.borrow_mut() = Rc::downgrade(&node);

        // right side
        // println!("{}, {}", total, c);
        let right = make_tree(&node_table, ind + 1, high, total - c);
        node.children.borrow_mut().push(Rc::clone(&right));
        *right.parent.borrow_mut() = Rc::downgrade(&node);
        println!("make tree from {}", node.code);
        node
    }
}

pub fn make_code(mut code_table: Vec<(u8, u8)>, node: &Rc<Node>, n: u8, code: u8) -> Vec<(u8, u8)> {
    // if a node has children, it is not any leaf.
    if node.children.borrow_mut().len() > 0 {
        // left: 0
        let code_table = make_code(code_table, &node.children.borrow()[0], n + 1, code << 1);
        // right: 1
        let code_table = make_code(code_table, &node.children.borrow()[1], n + 1, (code << 1) | 1);
        code_table
    } else {
        println!("{}, {}, {}", node.code, n, code);
        code_table[node.code as usize] =  (n, code);
        code_table
    }
}

pub fn write_tree(node: &Rc<Node>) -> Vec<bool> {
    let mut dst: Vec<bool> = Vec::new();
    if node.children.borrow_mut().len() == 0 {
        dst.push(true);
        dst.append(&mut to_bits(node.code));
    } else {
        dst.push(false);
        dst.append(&mut write_tree(&node.children.borrow()[0]));
        dst.append(&mut write_tree(&node.children.borrow()[1]));
    }
    dst
}

pub fn read_tree(src: &[u8]) -> Node {
    Node::new()
}
