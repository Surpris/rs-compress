//! integration_test
//! 
//! test functions


extern crate rs_compress;
use rs_compress as rsc;

use rand::prelude::*;

#[test]
fn test_simple_rle_encoding() {
    let a: Vec<u8> = vec![0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 4, 4, 4, 3];
    let n: usize = 3;
    let b = rsc::run_length_encoding::simple_rle::encode(&a, n);
    assert_eq!(b, [0, 0, 0, 2, 1, 1, 2, 2, 2, 1, 4, 4, 4, 0, 3]);
}

#[test]
fn test_simple_rle_decoding() {
    let a: Vec<u8> = vec![0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 4, 4, 4, 3];
    let n: usize = 3;
    let b = rsc::run_length_encoding::simple_rle::encode(&a, n);
    let c = rsc::run_length_encoding::simple_rle::decode(&b, n);
    assert_eq!(a, c);
}

#[test]
fn test_simple_rle_random() {
    let mut rng = rand::thread_rng();
    let mut a = [0u8; 1000];
    for _ in 0..100 {
        rng.fill_bytes(&mut a);
        let n: usize = 3;
        let b = rsc::run_length_encoding::simple_rle::encode(&a, n);
        let c = rsc::run_length_encoding::simple_rle::decode(&b, n);
        assert_eq!(a.to_vec(), c);
    }
}