//! alpha
//! 
//! alpha encoding

/// encode an integer
pub fn encode(v: u8) -> Vec<bool> {
    let mut dst: Vec<bool> = Vec::new();
    for _ in 0..v {
        dst.push(false);
    }
    dst.push(true);
    dst
}

/// decode an encoded integer
pub fn decode(bytes: &[bool]) -> u8 {
    assert!(bytes.len() - 1 <= u8::MAX as usize);
    let mut dst: u8 = 0u8;
    for b_ in bytes.iter() {
        if *b_ == true { break; }
        dst += 1;
    }
    dst
}
