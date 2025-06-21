//! Monkey Market

use std::iter::successors;

use hashbrown::HashMap;

use crate::util::parse::*;

type Input = Vec<i64>;

fn next(mut n: i64) -> i64 {
    n = ((n << 6) ^ n) & 0xFFFFFF;
    n = ((n >> 5) ^ n) & 0xFFFFFF;
    ((n << 11) ^ n) & 0xFFFFFF
}

pub fn parse(input: &str) -> Input {
    input.iter_signed().collect()
}

pub fn part1(input: &Input) -> i64 {
    input.iter().map(|&n| successors(Some(n), |&prev| Some(next(prev))).nth(2000).unwrap()).sum()
}

pub fn part2(input: &Input) -> i64 {
    let mut bananas = HashMap::with_capacity(10000);
    let mut seqs = HashMap::with_capacity(2000);

    for n in input {
        let mut n = *n;
        let mut seq = (0, 0, 0, 0);

        for i in 0..2000 {
            let prev_digit = n % 10;
            n = next(n);
            let curr_digit = n % 10;

            seq = (seq.1, seq.2, seq.3, curr_digit - prev_digit);

            if i >= 3 {
                seqs.entry(seq).or_insert(curr_digit);
            }
        }

        for (seq, count) in seqs.drain() {
            *bananas.entry(seq).or_insert(0) += count;
        }
    }

    *bananas.values().max().unwrap()
}
