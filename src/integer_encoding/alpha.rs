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
pub fn decode(mut src: Vec<bool>) -> (u8, Vec<bool>) {
    let mut value = 0u8;
    let mut dst: Vec<bool> = Vec::new();
    while src.remove(0) == false {
        value += 1;
    }
    while src.len() > 0 {
        dst.push(src.remove(0));
    }
    (value, dst)
}
