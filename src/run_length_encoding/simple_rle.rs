//! simple_rle
//!
//! Simple run-length encoding

const MAX_LEN: usize = 255;
const MIN_LEN: usize = 3;

/// encode a byte array
pub fn encode(fin: &[u8], n: usize) -> Vec<u8> {
    assert!(n >= MIN_LEN, "`n` must be > {}.", MIN_LEN);
    assert!(n <= MAX_LEN, "`n` must be > {}.", MIN_LEN);

    let mut fout: Vec<u8> = Vec::new();
    let mut current_index: usize = 0;
    let mut c: u8 = fin[current_index];
    while current_index < fin.len() {
        // number of the same characters in a sequence before another character
        let mut num: usize = 1;

        // count the number of the same characters
        let mut c1: u8 = 0;
        while num < MAX_LEN + n {
            current_index += 1;
            if current_index == fin.len() {
                break;
            }
            c1 = fin[current_index];
            if c != c1 {
                break;
            }
            num += 1;
        }
        // output the encoded values
        if num >= n {
            for _ in 0..n {
                fout.push(c);
            }
            fout.push((num - n) as u8);
        } else {
            for _ in 0..num {
                fout.push(c);
            }
        }
        // set the next character to count
        if num == MAX_LEN + n {
            current_index += 1;
            if current_index < fin.len() {
                c = fin[current_index];
            } else {
                c = c1;
            }
        } else {
            c = c1;
        }
    }
    fout
}

/// decode an encoded byte array
pub fn decode(fin: &[u8], n: usize) -> Vec<u8> {
    assert!(n >= MIN_LEN, "`n` must be > {}.", MIN_LEN);
    assert!(n <= MAX_LEN, "`n` must be > {}.", MIN_LEN);
    let mut fout: Vec<u8> = Vec::new();
    let mut current_index: usize = 0;
    let mut c: u8 = fin[current_index];
    while current_index < fin.len() {
        // number of the same characters in a sequence before another character
        let mut num: usize = 1;

        // count the number of the same characters
        let mut c1: u8 = 0;
        while num < n {
            current_index += 1;
            if current_index == fin.len() {
                break;
            }
            c1 = fin[current_index];
            if c != c1 {
                break;
            }
            num += 1;
        }
        if num == n {
            current_index += 1;
            if current_index < fin.len() {
                num += fin[current_index] as usize;
                current_index += 1;
                if current_index < fin.len() {
                    c1 = fin[current_index];
                }
            }
        }
        for _ in 0..num {
            fout.push(c);
        }
        c = c1;
    }
    fout
}
