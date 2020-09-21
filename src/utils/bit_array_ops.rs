//! bit_array_ops
//! 
//! functions for a bit array

/// convert a bit array into a byte arrays
pub fn to_bytes(src: &[bool]) -> Vec<u8> {
    assert_eq!(src.len() % 8, 0);
    let n_bytes = src.len() / 8;
    let mut dst: Vec<u8> = Vec::new();
    for ii in 0..n_bytes {
        dst.push(to_byte(&src[(ii*8)..(ii+1)*8]));
    }
    dst
}

/// convert a bit array into a byte
pub fn to_byte(src: &[bool]) -> u8 {
    assert_eq!(src.len(), 8);
    let mut value: u8 = 0u8;
    for jj in 0..8 {
        if src[jj] == true {
            value += 2u8.pow((8 - jj - 1) as u32);
        }
    }
    value
}

/// convert a bit array into a string
/// with binary expression
pub fn to_string(src: &[bool]) -> String {
    let mut dst = String::new();
    for b_ in src.to_vec() {
        if b_ == false {
            dst += "0";
        } else {
            dst += "1";
        }
    }
    dst
}
