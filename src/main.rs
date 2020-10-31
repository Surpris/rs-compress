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

fn main() {
    test_with_file();
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
                let mut file = File::open(entry.to_str().unwrap()).unwrap();
                let mut src: Vec<u8> = Vec::new();
                let _ = file.read_to_end(&mut src);
                let encoded: Vec<bool> = rsc::lz::lzss::encode(&src);
                println!("{}, {}", entry.to_str().unwrap(), encoded.len() / 8);
                let (decoded, residual): (Vec<u8>, Vec<bool>) = rsc::lz::lzss::decode(encoded);
                println!("{}, {}, {}", src.len(), decoded.len(), to_string(&residual));
                assert_eq!(src, decoded);
            }
        }
    } else {
        let mut file = File::open(path.to_str().unwrap()).unwrap();
        let mut src: Vec<u8> = Vec::new();
        let _ = file.read_to_end(&mut src);
        let encoded: Vec<bool> = rsc::lz::lzss::encode(&src);
        println!("{}, {}", path.to_str().unwrap(), encoded.len() / 8);
        let (decoded, residual): (Vec<u8>, Vec<bool>) = rsc::lz::lzss::decode(encoded);
        println!("{}, {}, {}", src.len(), decoded.len(), to_string(&residual));
        assert_eq!(src, decoded);
    }
    Ok(())
}

#[allow(dead_code)]
fn test_with_random_values() {
    let mut rng = rand::thread_rng();
    let mut a = [0u8; 1024];
    rng.fill_bytes(&mut a);
    let encoded: Vec<bool> = rsc::huffman_encoding::encode(&a);
    // println!("{}, {}", to_string(&encoded), encoded.len());
    let (decoded, residual): (Vec<u8>, Vec<bool>) = rsc::huffman_encoding::decode(encoded);
    println!("{}", to_string(&residual));
    // println!("{:?}, {}", decoded, to_string(&residual));
    println!("{}, {}", a.len(), decoded.len());
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
