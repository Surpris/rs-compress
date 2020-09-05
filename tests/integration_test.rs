//! integration_test
//! 
//! test functions


extern crate rs_compress;
use rs_compress as rsc;

use rand::prelude::*;

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
    let mut a = [0u8; 10000];
    let n: usize = 3;
    for _ in 0..1000 {
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
    let mut a = [0u8; 10000];
    for _ in 0..1000 {
        rng.fill_bytes(&mut a);
        let b = rsc::run_length_encoding::switched_rle::encode(&a);
        let c = rsc::run_length_encoding::switched_rle::decode(&b);
        assert_eq!(a.to_vec(), c);
    }
}