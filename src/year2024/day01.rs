//! # Historian Hysteria
//!
use hashbrown::HashMap;

use crate::util::parse::*;

type Input = (Vec<u32>, Vec<u32>);

pub fn parse(input: &str) -> Input {
    input.iter_unsigned::<u32>().array_chunks::<2>().map(|[l, r]| (l, r)).unzip()
}

pub fn part1(input: &Input) -> u32 {
    let (mut left, mut right) = input.clone();

    left.sort();
    right.sort();

    left.iter().zip(right.iter()).fold(0, |acc, (l, r)| acc + r.abs_diff(*l))
}

pub fn part2(input: &Input) -> u32 {
    let (left, right) = &input;

    let mut occur = HashMap::new();
    for value in right {
        occur.entry(value).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut sum = 0;
    for value in left {
        sum += value * occur.get(&value).unwrap_or(&0);
    }

    sum
}
