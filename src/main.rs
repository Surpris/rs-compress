//! rs-compress

extern crate rs_compress;
use rs_compress as rsc;

fn main() {
    let a: Vec<u8> = vec![0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 4, 4, 1];
    let n: usize = 3;
    // let b = rsc::run_length_encoding::simple_rle::run_length_encode(&a, n);
    // println!("{:?}", b);
}
