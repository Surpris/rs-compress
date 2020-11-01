//! # LZH encoding
//!
//!

use crate::huffman_encoding::tree;
use crate::utils::bit_length_of_value;
use crate::utils::bit_value_ops;
use crate::utils::u64_value_ops;
use std::collections::HashMap;

// minimum length of target to encode
const MIN_LEN_OF_TARGET: usize = 4;
// maximum length of target to encode
const MAX_LEN_OF_TARGET: usize = 256;
// number of bits expressing the position of an encoding slide
const POSITION_BITS: u64 = 16;
// length of window
const WINDOW_LEN: usize = 1 << POSITION_BITS;
// limit of window size
const WINDOW_LIMIT: usize = WINDOW_LEN * 2;
// length of (ref + target)
const LEN_OF_MOVING: usize = WINDOW_LEN + MAX_LEN_OF_TARGET;
// buffer size
const NIL: usize = WINDOW_LIMIT + MAX_LEN_OF_TARGET;

// number of bits expressing the size of a buffer for Huffman encoding
const NBR_OF_HUFF_BITS: u64 = 14;
// size of a buffer for Huffman encoding
const HUFF_LEN: usize = 1 << NBR_OF_HUFF_BITS;
// code length
const CODE_LEN: u64 = 256;
// number of bits used to encode a symbol and the length with Huffman encoding
const NBR_OF_BITS_SYM_LEN: u64 = 9;
// number of bits used to encode the position with Huffman encoding
const NBR_OF_BITS_POS: u64 = 5;

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
    *match_len = 0;
    *match_pos = 0;
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
    src: &mut Vec<u8>,
    buff: &mut [u8],
    next_pos: &mut [usize],
    ht: &mut HashMap<u64, usize>,
    data_size: &mut usize,
    start_point: &mut usize,
) {
    if *data_size < WINDOW_LIMIT + MAX_LEN_OF_TARGET {
        return;
    }
    for ii in 0..LEN_OF_MOVING {
        buff[ii] = buff[ii + WINDOW_LEN];
    }
    let mut size_: usize = LEN_OF_MOVING;
    for ii in LEN_OF_MOVING..(LEN_OF_MOVING + src.len()) {
        buff[ii] = src.remove(0);
        size_ += 1;
        if size_ >= buff.len() {
            break;
        }
        if src.len() == 0 {
            break;
        }
    }
    *data_size = size_;

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

/// encode a byte array
pub fn encode(src: &[u8]) -> Vec<bool> {
    if src.len() == 0 {
        return Vec::new();
    }
    // initialize
    let mut src_vec: Vec<u8> = src.to_vec();
    let mut start_point: usize = 0;
    let mut match_len: usize = 0;
    let mut match_pos: usize = 0;
    let mut next_pos: [usize; WINDOW_LEN] = [0; WINDOW_LEN];
    let mut ht: HashMap<u64, usize> = HashMap::new();
    let mut huffman_buff: [(u64, u64); HUFF_LEN] = [(0u64, 0u64); HUFF_LEN];
    let mut huffman_count: usize = 0;
    let mut dst: Vec<bool> = u64_value_ops::to_bits(src.len() as u64);

    // first buffering
    let mut buff: [u8; NIL] = [0u8; NIL];
    let mut data_size: usize = 0;
    for ii in 0..NIL {
        buff[ii] = src_vec.remove(0);
        data_size += 1;
        if src_vec.len() == 0 {
            break;
        }
    }

    // main loop
    while start_point < data_size {
        let num: usize;
        get_matched_pattern(
            &buff,
            &next_pos,
            &ht,
            data_size,
            start_point,
            &mut match_len,
            &mut match_pos,
        );
        if match_len < MIN_LEN_OF_TARGET {
            num = 1;
            huffman_buff[huffman_count] = (buff[start_point] as u64, 0);
        } else {
            num = match_len;
            huffman_buff[huffman_count] = (
                CODE_LEN + (num - MIN_LEN_OF_TARGET) as u64,
                (start_point - match_pos - 1) as u64,
            );
        }
        huffman_count += 1;
        if huffman_count >= HUFF_LEN {
            dst.append(&mut huffman_encode(&huffman_buff, huffman_count));
            huffman_count = 0;
        }
        for _ in 0..num {
            insert_recode(&buff, &mut next_pos, &mut ht, start_point);
            start_point += 1;
            if start_point >= WINDOW_LIMIT {
                update_values(
                    &mut src_vec,
                    &mut buff,
                    &mut next_pos,
                    &mut ht,
                    &mut data_size,
                    &mut start_point,
                )
            }
        }
    }
    if huffman_count > 0 {
        dst.append(&mut huffman_encode(&huffman_buff, huffman_count));
    }
    dst
}

fn huffman_encode(buff: &[(u64, u64)], size: usize) -> Vec<bool> {
    let mut ft_sym_len: Vec<u64> =
        vec![0u64; CODE_LEN as usize + MAX_LEN_OF_TARGET - MIN_LEN_OF_TARGET + 1];
    let mut ft_pos: Vec<u64> = vec![0u64; (POSITION_BITS + 1) as usize];
    for ii in 0..size {
        ft_sym_len[buff[ii].0 as usize] += 1;
        if buff[ii].0 >= CODE_LEN {
            ft_pos[bit_length_of_value::<u64, u64>(buff[ii].1) as usize] += 1;
        }
    }

    let tree_sym_len = tree::make_tree(&ft_sym_len);
    let ct_sym_len = tree::make_code(&tree_sym_len);
    let tree_pos = tree::make_tree(&ft_pos);
    let ct_pos = tree::make_code(&tree_pos);
    let mut dst: Vec<bool> = Vec::new();
    dst.append(&mut u64_value_ops::to_n_bits((size - 1) as u64, NBR_OF_HUFF_BITS));
    dst.append(&mut tree::tree_to_bits(&tree_sym_len, NBR_OF_BITS_SYM_LEN));
    dst.append(&mut tree::tree_to_bits(&tree_pos, NBR_OF_BITS_POS));
    for ii in 0..size {
        let (sl, p) = buff[ii];
        let &(code, n) = ct_sym_len.get(&sl).unwrap();
        dst.append(&mut u64_value_ops::to_n_bits(code, n));
        if sl >= CODE_LEN {
            let bits: u64 = bit_length_of_value(p);
            let &(code, n) = ct_pos.get(&bits).unwrap();
            dst.append(&mut u64_value_ops::to_n_bits(code, n));
            if n > 0 {
                dst.append(&mut u64_value_ops::to_n_bits(p + 1, n));
            }
        }
    }
    dst
}

/// decode a byte array
pub fn decode(mut src: Vec<bool>) -> (Vec<u8>, Vec<bool>) {
    if src.len() < 64 {
        return (Vec::new(), src);
    }
    let mut buff: Vec<bool> = Vec::new();
    for _ in 0..64 {
        buff.push(src.remove(0));
    }
    let mut size = bit_value_ops::to_u64(&buff);
    let mut buffer = [0u8; WINDOW_LEN];
    let mut start_point: usize = 0;
    let mut huffman_buff: [(u64, u64); HUFF_LEN] = [(0u64, 0u64); HUFF_LEN];
    let mut huffman_size: usize = 0;
    let mut huffman_count: usize = 0;
    let mut dst: Vec<u8> = Vec::new();
    while size > 0 {
        let num;
        if huffman_count == huffman_size {
            let (size_, buff_) = huffman_decode(src, &mut huffman_buff);
            huffman_size = size_;
            huffman_count = 0;
            src = buff_;
        }
        if huffman_buff[huffman_count].0 >= CODE_LEN {
            num = huffman_buff[huffman_count].0 - CODE_LEN + MIN_LEN_OF_TARGET as u64;
            let mut pos: usize = (huffman_buff[huffman_count].1 + 1) as usize;
            pos = if start_point >= pos {
                start_point - pos
            } else {
                start_point + WINDOW_LEN - pos
            };
            for _ in 0..num {
                let c = buffer[pos];
                dst.push(c);
                buffer[start_point] = c;
                pos += 1;
                start_point += 1;
                if pos >= WINDOW_LEN {
                    pos = 0;
                }
                if start_point >= WINDOW_LEN {
                    start_point = 0;
                }
            }
        } else {
            num = 1;
            let c: u8 = huffman_buff[huffman_count].0 as u8;
            dst.push(c);
            buffer[start_point] = c;
            start_point += 1;
            if start_point >= WINDOW_LEN {
                start_point = 0;
            }
        }
        huffman_count += 1;
        size -= num;
    }
    (dst, src)
}

fn huffman_decode(mut src: Vec<bool>, huff_buff: &mut [(u64, u64)]) -> (usize, Vec<bool>) {
    let mut buff: Vec<bool> = Vec::new();
    for _ in 0..NBR_OF_HUFF_BITS {
        buff.push(src.remove(0));
    }
    let size: usize = bit_value_ops::to_u64(&buff) as usize + 1;
    let (tree_sym_len, src) = tree::bits_to_tree(src, NBR_OF_BITS_SYM_LEN);
    let (tree_pos, mut src) = tree::bits_to_tree(src, NBR_OF_BITS_POS);
    for ii in 0..size {
        let (code, buff) = tree::search_code(&tree_sym_len, src);
        src = buff;
        let p = if code >= CODE_LEN {
            let (n, buff1) = tree::search_code(&tree_pos, src);
            src = buff1;
            if n > 0 {
                let mut buff2: Vec<bool> = Vec::new();
                for _ in 0..n {
                    buff2.push(src.remove(0));
                }
                (1 << n) + bit_value_ops::to_u64(&buff2) - 1
            } else {
                n
            }
        } else {
            0
        };
        huff_buff[ii] = (code, p);
    }
    (size, src)
}
