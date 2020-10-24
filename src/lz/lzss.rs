//! # LZSS encoding
//!
//!

use std::collections::HashMap;

// length of bits expressing the length of target
const LEN_OF_BITS: u64 = 4;
// minimum length of target to encode
const MIN_LEN_OF_TARGET: usize = 3;
// maximum length of target to encode
// = MIN_LEN_OF_TARGET + 2**LEN_OF_BITS - 1
const MAX_LEN_OF_TARGET: usize = 18;
// length of bits expressin the position of an encoding slide
const POSITION_BITS: u64 = 13;
// length of window
const WINDOW_LEN: usize = 1 << POSITION_BITS;
// limit of window size
const WINDOW_LIMIT: usize = WINDOW_LEN * 2;
// length of (ref + target)
const LEN_OF_MOVING: usize = WINDOW_LEN + MAX_LEN_OF_TARGET;
// buffer size
const NIL: usize = WINDOW_LIMIT + MAX_LEN_OF_TARGET;

/// get the hash value corresponding to the starting point of encoding
///
/// # Parameters
/// * buff: buffer for encoding
/// * start_point: starting point of encoding
fn get_hash_value(buff: &[u8], start_point: usize) -> u64 {
    let mut value: u64 = 0u64;
    for ii in 0..MIN_LEN_OF_TARGET {
        value = (value << 8) + buff[start_point + ii] as u64;
    }
    value
}

/// insert a recode including the starting point to a hash table
///
/// # Parameters
/// * buff: buffer for encoding
/// * next_pos: connected list of the starting points
/// * ht: hash table of the starting points
/// * start_point: starting point of encoding
fn insert_recode(
    buff: &[u8],
    next_pos: &mut [usize],
    ht: &mut HashMap<u64, usize>,
    start_point: usize,
) {
    let hv: u64 = get_hash_value(buff, start_point);
    match ht.get(&hv) {
        Some(x) if *x != NIL => next_pos[start_point & (WINDOW_LEN - 1)] = *x,
        _ => next_pos[start_point & (WINDOW_LEN - 1)] = NIL,
    }
    ht.insert(hv, start_point);
}

/// get the matched pattern
///
/// # Parameters
/// * buff
/// * next_pos
/// * ht
/// * data_size
/// * start_point
/// * match_len
/// * match_pos
fn get_matched_pattern(
    buff: &[u8],
    next_pos: &[usize],
    ht: &HashMap<u64, usize>,
    data_size: usize,
    start_point: usize,
    match_len: &mut usize,
    match_pos: &mut usize,
) {
    let hv: u64 = get_hash_value(buff, start_point);
    let limit: usize = if start_point < WINDOW_LEN {
        0
    } else {
        start_point - WINDOW_LEN
    };
    if let Some(matched) = ht.get(&hv) {
        let mut nbr = *matched;
        while nbr != NIL && nbr >= limit {
            if buff[start_point + *match_len] == buff[nbr + *match_len] {
                let mut x: usize = 0;
                while x < MAX_LEN_OF_TARGET {
                    if buff[start_point + x] != buff[nbr + x] {
                        break;
                    }
                    x += 1;
                }
                if *match_len < x {
                    *match_len = x;
                    *match_pos = nbr;
                    if x == MAX_LEN_OF_TARGET {
                        break;
                    }
                }
            }
            nbr = next_pos[nbr & (WINDOW_LEN - 1)];
        }
        if *match_len != 0 && *match_len >= data_size - start_point {
            *match_len = data_size - start_point;
        }
    }
}

/// update inner parameters
///
/// # Parameters
/// * src
/// * buff
/// * next_pos
/// * ht
/// * data_size
/// * start_point
/// * match_len
/// * match_pos
fn update_values(
    src: &[u8],
    buff: &mut [u8],
    next_pos: &mut [usize],
    ht: &mut HashMap<u64, usize>,
    data_size: &mut usize,
    start_point: &mut usize,
    match_len: &mut usize,
    match_pos: &mut usize,
) {
    if *data_size < WINDOW_LIMIT + MAX_LEN_OF_TARGET {
        return;
    }
    for ii in 0..LEN_OF_MOVING {
        buff[ii] = buff[ii + WINDOW_LEN];
    }
    let mut size_: usize = LEN_OF_MOVING;
    while size_ < LEN_OF_MOVING + src.len() {
        buff[size_] = src[size_ - LEN_OF_MOVING];
        size_ += 1;
        if size_ > buff.len() {
            break;
        }
    }
    *data_size = size_ - 1;

    let mut remove_keys: Vec<u64> = Vec::new();
    for (k, v) in ht.iter_mut() {
        if *v < WINDOW_LEN {
            remove_keys.push(*k);
        } else if *v != NIL {
            *v -= WINDOW_LEN;
        }
    }
    for k in remove_keys {
        ht.remove(&k);
    }

    for ii in 0..WINDOW_LEN {
        if next_pos[ii] != NIL && next_pos[ii] > WINDOW_LEN {
            next_pos[ii] -= WINDOW_LEN;
        } else {
            next_pos[ii] = NIL;
        }
    }
    *start_point = *start_point - WINDOW_LEN;
}

pub fn encode(src: &[u8]) -> Vec<bool> {
    Vec::new()
}

pub fn decode(mut src: Vec<bool>) -> (Vec<u8>, Vec<bool>) {
    (Vec::new(), src)
}
