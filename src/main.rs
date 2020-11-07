//! rs-compress

extern crate rs_compress;
use rs_compress as rsc;
use rsc::utils::bit_value_ops::to_string;

use rand::prelude::*;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

macro_rules! measure {
    ( $x:expr) => {{
        let start = Instant::now();
        let result = $x;
        let end = start.elapsed();
        println!("{}.{:03} sec elapsed.", end.as_secs(), end.subsec_millis());
        result
    }};
}

fn main() {
    measure!({
        test_with_file();
    })
    // test_with_random_values();
}

#[allow(dead_code)]
fn test_with_file() {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() > 1);
    match Path::new(&args[1]).canonicalize() {
        Ok(pb) => match walk_and_encode(&pb) {
            Ok(_) => return,
            Err(err) => println!("{}", err.to_string()),
        },
        Err(err) => println!("{}", err.to_string()),
    }
}

#[allow(dead_code)]
fn walk_and_encode(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        let mut entries = fs::read_dir(path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        entries.sort();
        for entry in entries {
            if entry.is_dir() {
                walk_and_encode(&entry)?;
            } else {
                encode_decode(&entry);
            }
        }
    } else {
        encode_decode(&path);
    }
    Ok(())
}

#[allow(dead_code)]
fn encode_decode(path: &Path) {
    let mut file = File::open(path.to_str().unwrap()).unwrap();
    let mut src: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut src);
    let encoded: Vec<bool> = rsc::lz::lzh::encode(&src);
    println!("{}, {}", path.to_str().unwrap(), encoded.len() / 8);
    let (decoded, residual): (Vec<u8>, Vec<bool>) = rsc::lz::lzh::decode(encoded);
    println!("{}, {}, {}", src.len(), decoded.len(), residual.len());
    assert_eq!(src, decoded);
}

#[allow(dead_code)]
fn test_integer_encoding() {
    for ii in 0..255 {
        let encoded: Vec<bool> = rsc::integer_encoding::delta::encode::<u64>(ii as u64);
        // let encoded_u8: Vec<bool> = rsc::integer_encoding::delta::encode_u8(ii as u8);
        // assert_eq!(encoded, encoded_u8);
        let (decoded, _res) = rsc::integer_encoding::delta::decode::<u64>(encoded.clone());
        // let (decoded_u8, _res) = rsc::integer_encoding::delta::decode_u8(encoded);
        // assert_eq!(ii, decoded_u8);
        assert_eq!(ii, decoded as u8);
    }
}

#[allow(dead_code)]
fn test_with_random_values() {
    let mut rng = rand::thread_rng();
    let mut a = [0u8; 1024];
    rng.fill_bytes(&mut a);
    let encoded: Vec<bool> = rsc::lz::lzh::encode(&a);
    println!("{}, {}", to_string(&encoded), encoded.len() / 8);
    let (decoded, residual): (Vec<u8>, Vec<bool>) = rsc::lz::lzh::decode(encoded);
    println!("{}, {}, {}", a.len(), decoded.len(), residual.len());
    assert_eq!(a.to_vec(), decoded);
}

#[allow(dead_code)]
fn test_with_string() {
    let src: String = String::from("abccddeeeeffffgggggggghhhhhhhh");
    let encoded: Vec<bool> = rsc::huffman_encoding::encode(&src.as_bytes());
    let (decoded, encoded): (Vec<u8>, Vec<bool>) = rsc::huffman_encoding::decode(encoded);
    println!("{}", decoded.iter().map(|&s| s as char).collect::<String>());
    println!("{:?}", encoded);
}

#[allow(dead_code)]
fn rng_fill_u64(a: &mut [u64]) {
    let mut rng = rand::thread_rng();
    for ii in 0..a.len() {
        a[ii] = rng.next_u64();
    }
}
