//! integration_test
//! 
//! test functions


extern crate rs_compress;
use rs_compress as rsc;

// use rand::Rng;

#[test]
fn test_simple_rle_encoding() {
    let a: Vec<u8> = vec![0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 4, 4, 1];
    let n: usize = 3;
    let b = rsc::run_length_encoding::simple_rle::encode(&a, n);
    assert_eq!(b, [0, 0, 0, 2, 1, 1, 2, 2, 2, 1, 4, 4, 1]);
}