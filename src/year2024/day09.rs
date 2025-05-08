//! # Disk Fragmenter
//!

use std::iter::{from_fn, repeat_n};

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> usize {
    let mut uncompressed =
        input.lines().next().unwrap().chars().map(|length| length as usize - 48).enumerate().fold(
            Vec::new(),
            |mut acc, (idx, length)| {
                if idx % 2 == 0 {
                    let file_id = idx / 2;
                    acc.extend(repeat_n(Some(file_id), length));
                } else {
                    acc.extend(repeat_n(None, length));
                }
                acc
            },
        );

    let mut answer: usize = 0;
    let mut pos = 0;

    while pos < uncompressed.len() {
        let file_id = if let Some(file_id) = uncompressed[pos] {
            file_id
        } else {
            from_fn(|| uncompressed.pop()).find_map(|x| x).unwrap()
        };
        answer += pos * file_id;
        pos += 1;
    }

    answer
}

pub fn part2(input: &str) -> usize {
    let mut compressed =
        input.lines().next().unwrap().chars().map(|length| length as usize - 48).enumerate().fold(
            Vec::new(),
            |mut acc, (idx, size)| {
                if idx % 2 == 0 {
                    let file_id = idx / 2;
                    acc.push((Some(file_id), size));
                } else {
                    acc.push((None, size));
                }
                acc
            },
        );

    let mut idx_rev = compressed.len() - 1;
    while idx_rev > 0 {
        if let (Some(_id), file_size) = compressed[idx_rev] {
            for idx in 0..compressed.len() {
                if idx >= idx_rev {
                    break;
                }

                if let (None, free_size) = compressed[idx] {
                    if free_size > file_size {
                        compressed.insert(idx + 1, (None, free_size - file_size));
                        idx_rev += 1;
                    }
                    if free_size >= file_size {
                        compressed[idx] = compressed[idx_rev];
                        compressed[idx_rev] = (None, file_size);
                        break;
                    }
                }
            }
        }
        idx_rev -= 1;
    }

    let mut answer = 0;
    let mut pos = 0;

    for block in compressed {
        if let (Some(file_id), file_size) = block {
            for _ in 0..file_size {
                answer += pos * file_id;
                pos += 1;
            }
        } else {
            pos += block.1;
        }
    }

    answer
}
