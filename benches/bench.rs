#![feature(test)]

extern crate test;
extern crate rs_compress;
use rs_compress as rsc;

use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

const TARGET_PATH: &str = "";

#[bench]
fn bench_encode_decode(b: &mut test::Bencher) {
    b.iter(|| {
        test_with_file();
    });
}

fn test_with_file() {
    match Path::new(TARGET_PATH).canonicalize() {
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
                encode_decode(&entry);
            }
        }
    } else {
        encode_decode(&path);
    }
    Ok(())
}

fn encode_decode(path: &Path) {
    let mut file = File::open(path.to_str().unwrap()).unwrap();
    let mut src: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut src);
    let encoded: Vec<bool> = rsc::lz::lzh::encode(&src);
    let (decoded, _residual): (Vec<u8>, Vec<bool>) = rsc::lz::lzh::decode(encoded);
    assert_eq!(src, decoded);
}
