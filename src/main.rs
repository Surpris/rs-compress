//! rs-compress

extern crate rs_compress;
use rs_compress as rsc;

// use bit_vec::BitVec;
// use rand::prelude::*;

fn main() {
    for v in 0..255 {
        let b: Vec<bool> = rsc::integer_encoding::gamma::encode(v);
        println!("{}, {}", v, print_bool_vec(&b));
        let (c, dst): (u8, Vec<bool>) = rsc::integer_encoding::gamma::decode(b);
        println!("{}, {}", c, print_bool_vec(&dst));
    }
    let v = 255u8;
    let b: Vec<bool> = rsc::integer_encoding::gamma::encode(v);
    println!("{}, {}", v, print_bool_vec(&b));
    let (c, dst): (u8, Vec<bool>) = rsc::integer_encoding::gamma::decode(b);
    println!("{}, {}", c, print_bool_vec(&dst));
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