//! zle
//!
//! Zero length encoding

/// encode a byte array
pub fn encode(src: &[u8]) -> Vec<u8> {
    let mut current_index: usize = 0;
    let mut dst: Vec<u8> = Vec::new();
    while current_index < src.len() {
        let mut c: u8 = src[current_index];
        if c == 0 {
            let mut count = 0;
            while c == 0 {
                count += 1;
                current_index += 1;
                if current_index >= src.len() {
                    break;
                }
                c = src[current_index];
            }
            count += 1;
            while count != 1 {
                dst.push(count & 1);
                count >>= 1;
            }
        } else {
            if c == 0xfe {
                dst.push(0xff);
                dst.push(0x00);
            } else if c == 0xff {
                dst.push(0xff);
                dst.push(0x01);
            } else {
                dst.push(c + 1);
            }
            current_index += 1;
        }
    }
    dst
}

/// decode an encoded byte array
pub fn decode(src: &[u8]) -> Vec<u8> {
    let mut current_index: usize = 0;
    let mut dst: Vec<u8> = Vec::new();
    let mut buff: Vec<u8> = Vec::new();
    while current_index < src.len() {
        let mut c: u8 = src[current_index];
        if c <= 1 {
            let mut count: usize = 1;
            buff.push(c);
            loop {
                current_index += 1;
                if current_index >= src.len() {
                    break;
                }
                c = src[current_index];
                if c > 1 {
                    break;
                }
                buff.push(c);
            }
            while let Some(x) = buff.pop() {
                count = (count << 1) + x as usize;
            }
            for _ in 0..(count - 1) {
                dst.push(0x00);
            }
        } else {
            if c == 0xff {
                current_index += 1;
                if current_index >= src.len() {
                    break;
                }
                c = src[current_index];
                if c == 0x00 {
                    dst.push(0xfe);
                } else {
                    dst.push(0xff);
                }
            } else {
                dst.push(c - 1);
            }
            current_index += 1;
        }
    }
    dst
}
