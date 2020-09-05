//! rs-compress

extern crate rs_compress;
use rs_compress as rsc;


use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut a = [0u8; 10];
    for _ in 0..100 {
        rng.fill_bytes(&mut a);
        println!("{:?}", a);
        let n: usize = 3;
        let b = rsc::run_length_encoding::simple_rle::encode(&a, n);
        let c = rsc::run_length_encoding::simple_rle::decode(&b, n);
        assert_eq!(a.to_vec(), c);
    }
}
