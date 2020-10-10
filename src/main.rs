//! rs-compress

extern crate rs_compress;
use rs_compress as rsc;
use rsc::utils::bit_value_ops::to_string;

use rand::prelude::*;
use std::collections::BTreeMap;
use std::ops::Bound::Included;

fn main() {
    test_huffman_encoding();
}

#[allow(dead_code)]
fn test_huffman_encoding() {
    let mut rng = rand::thread_rng();
    let mut a = [0u8; 1024];
    rng.fill_bytes(&mut a);
    // println!("{:?}", a.to_vec());
    let encoded: Vec<bool> = rsc::huffman_encoding::encode(&a);
    // println!("{}, {}", to_string(&encoded), encoded.len());
    let (decoded, _residual): (Vec<u8>, Vec<bool>) = rsc::huffman_encoding::decode(encoded);
    println!("{}", to_string(&_residual));
    // println!("{:?}, {}", decoded, to_string(&residual));
    println!("{}, {}", a.len(), decoded.len());
    assert_eq!(a.to_vec(), decoded);
}

#[allow(dead_code)]
fn test_huffman_encoding_1() {
    let src: String = String::from("abccddeeeeffffgggggggghhhhhhhh");
    let encoded: Vec<bool> = rsc::huffman_encoding::encode(&src.as_bytes());
    let (decoded, encoded): (Vec<u8>, Vec<bool>) = rsc::huffman_encoding::decode(encoded);
    println!("{}", decoded.iter().map(|&s| s as char).collect::<String>());
    println!("{:?}", encoded);
}

#[allow(dead_code)]
fn test_b_tree_map() {
    let mut tree: BTreeMap<u32, u32> = BTreeMap::new();
    for ii in 0..256 {
        tree.insert(ii, 0);
    }
    let lower: Vec<(u32, u32)> = tree.range((Included(&0), Included(&128))).map(|(&k, &v)| (k, v)).collect();
    println!("{:?}",  lower);
}

#[allow(dead_code)]
fn test_print_tree() {
    let code: Vec<u8> = vec![7, 6, 5, 4, 3, 2, 1, 0];
    let counts: Vec<u32> = vec![8, 8, 4, 4, 2, 2, 1, 1];
    let mut src: Vec<u8> = Vec::new();
    for ii in 0..code.len() {
        for _ in 0..counts[ii] {
            src.push(code[ii]);
        }
    }
    let _encoded: Vec<bool> = rsc::shannon_fano_encoding::encode(&src);
}

#[allow(dead_code)]
fn test_shannon_fano_simple() {
    let src: Vec<u8> = vec![3, 3, 3, 3, 1, 1, 1, 2, 2];
    let encoded: Vec<bool> = rsc::shannon_fano_encoding::encode(&src);
    println!("{}, {}", to_string(&encoded), encoded.len());
    let (decoded, residual): (Vec<u8>, Vec<bool>) = rsc::shannon_fano_encoding::decode(encoded);
    println!("{:?}, {}", decoded, to_string(&residual));
}

#[allow(dead_code)]
fn test_shannon_fano_increment(n: u32) {
    let mut a: Vec<u8> = Vec::new();
    for ii in 0..n {
        a.push((ii % 256) as u8);
    }
    let encoded: Vec<bool> = rsc::shannon_fano_encoding::encode(&a);
    let (_decoded, residual): (Vec<u8>, Vec<bool>) = rsc::shannon_fano_encoding::decode(encoded);
    println!("{}", to_string(&residual));
    // println!("{:?}, {}", decoded, to_string(&residual));
}

#[allow(dead_code)]
fn test_shannon_fano() {
    let mut rng = rand::thread_rng();
    let mut a = [0u8; 256];
    rng.fill_bytes(&mut a);
    // println!("{:?}", a.to_vec());
    let encoded: Vec<bool> = rsc::shannon_fano_encoding::encode(&a);
    // println!("{}, {}", to_string(&encoded), encoded.len());
    let (decoded, _residual): (Vec<u8>, Vec<bool>) = rsc::shannon_fano_encoding::decode(encoded);
    println!("{}", to_string(&_residual));
    // println!("{:?}, {}", decoded, to_string(&residual));
    println!("{}, {}", a.len(), decoded.len());
    // assert_eq!(a.to_vec(), decoded);
}
