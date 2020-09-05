//! integration_test
//! 
//! test functions


extern crate rs_compress;
use rs_compress as rsc;

use rand::prelude::*;

const SRC_LENGTH: usize = 1000;
const NBR_LOOPS: usize = 1000;

#[test]
fn test_simple_rle_encode() {
    let a: Vec<u8> = vec![0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 4, 4, 4, 3];
    let n: usize = 3;
    let b = rsc::run_length_encoding::simple_rle::encode(&a, n);
    assert_eq!(b, [0, 0, 0, 2, 1, 1, 2, 2, 2, 1, 4, 4, 4, 0, 3]);
}

#[test]
fn test_simple_rle_decode() {
    let a: Vec<u8> = vec![0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 4, 4, 4, 3];
    let n: usize = 3;
    let b = rsc::run_length_encoding::simple_rle::encode(&a, n);
    let c = rsc::run_length_encoding::simple_rle::decode(&b, n);
    assert_eq!(a, c);
}

#[test]
fn test_simple_rle_random() {
    let mut rng = rand::thread_rng();
    let mut a = [0u8; SRC_LENGTH];
    let n: usize = 3;
    for _ in 0..NBR_LOOPS {
        rng.fill_bytes(&mut a);
        let b = rsc::run_length_encoding::simple_rle::encode(&a, n);
        let c = rsc::run_length_encoding::simple_rle::decode(&b, n);
        assert_eq!(a.to_vec(), c);
    }
}

#[test]
fn test_switched_rle_encode() {
    let a: Vec<u8> = vec![0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 4, 4, 4, 3];
    let b = rsc::run_length_encoding::switched_rle::encode(&a);
    assert_eq!(b, [1, 0, 4, 1, 1, 1, 1, 2, 3, 1, 4, 2, 1, 3]);
}

#[test]
fn test_switched_rle_decode() {
    let a: Vec<u8> = vec![0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 4, 4, 4, 3];
    let b = rsc::run_length_encoding::switched_rle::encode(&a);
    let c = rsc::run_length_encoding::switched_rle::decode(&b);
    assert_eq!(c, a);
}

#[test]
fn test_switched_rle_random() {
    let mut rng = rand::thread_rng();
    let mut a = [0u8; SRC_LENGTH];
    for _ in 0..NBR_LOOPS {
        rng.fill_bytes(&mut a);
        let b = rsc::run_length_encoding::switched_rle::encode(&a);
        let c = rsc::run_length_encoding::switched_rle::decode(&b);
        assert_eq!(a.to_vec(), c);
    }
}

#[test]
fn test_zle_encode() {
    let a: Vec<u8> = vec![3, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 0, 0, 0, 3];
    let b = rsc::run_length_encoding::zle::encode(&a);
    assert_eq!(b, [4, 1, 0, 2, 2, 3, 3, 3, 3, 0, 0, 4]);
}

#[test]
fn test_zle_decode() {
    let a: Vec<u8> = vec![3, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 0, 0, 0, 3];
    let b = rsc::run_length_encoding::zle::encode(&a);
    let c = rsc::run_length_encoding::zle::decode(&b);
    assert_eq!(c, a);
}

#[test]
fn test_zle_random() {
    let mut rng = rand::thread_rng();
    let mut a = [0u8; SRC_LENGTH];
    for _ in 0..NBR_LOOPS {
        rng.fill_bytes(&mut a);
        let b = rsc::run_length_encoding::zle::encode(&a);
        let c = rsc::run_length_encoding::zle::decode(&b);
        assert_eq!(a.to_vec(), c);
    }
}