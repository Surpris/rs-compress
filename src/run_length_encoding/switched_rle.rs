//! switched_rle
//!
//! Switched run-length encoding

const MAX_LEN: usize = 255;
const MODE_LITERAL: usize = 1;
const MODE_FILL: usize = 0;

/// encode a byte array
pub fn encode(src: &[u8]) -> Vec<u8> {
    let mut mode: usize = MODE_LITERAL;
    let mut current_index: usize = 0;
    let mut c: u8 = src[current_index];
    let mut count: usize = 0;
    let mut dst: Vec<u8> = Vec::new();
    while current_index < src.len() {
        let mut c1: u8 = 0;
        if mode == MODE_LITERAL {
            let mut buff: Vec<u8> = Vec::new();
            buff.push(c);
            while buff.len() < MAX_LEN {
                current_index += 1;
                if current_index >= src.len() {
                    break;
                }
                c1 = src[current_index];
                if c1 == c {
                    break;
                }
                buff.push(c1);
                c = c1;
            }
            dst.push(buff.len() as u8);
            for c_ in buff.iter() {
                dst.push(*c_);
            }
            if buff.len() == MAX_LEN {
                current_index += 1;
                if current_index < src.len() {
                    c = src[current_index];
                }
            } else {
                mode = MODE_FILL;
                c = c1;
                count = 1;
            }
        } else if mode == MODE_FILL {
            while count < MAX_LEN {
                current_index += 1;
                if current_index >= src.len() {
                    break;
                }
                c1 = src[current_index];
                if c != c1 {
                    break;
                }
                count += 1;
            }
            dst.push(count as u8);
            if count == MAX_LEN {
                count = 0;
            } else {
                mode = MODE_LITERAL;
                c = c1;
            }
        }
    }
    dst
}

/// decode an encoded byte array
pub fn decode(src: &[u8]) -> Vec<u8> {
    let mut mode: usize = MODE_LITERAL;
    let mut current_index: usize = 0;
    let mut c: u8 = 0;
    let mut dst: Vec<u8> = Vec::new();
    while current_index < src.len() {
        let num: u8 = src[current_index];
        if mode == MODE_LITERAL {
            for _ in 0..num {
                current_index += 1;
                if current_index >= src.len() {
                    break;
                }
                c = src[current_index];
                dst.push(c);
            }
            if num < MAX_LEN as u8 {
                mode = MODE_FILL;
            }
        } else if mode == MODE_FILL {
            for _ in 0..num {
                dst.push(c);
            }
            if num < MAX_LEN as u8 {
                mode = MODE_LITERAL;
            }
        }
        current_index += 1;
    }
    dst
}
