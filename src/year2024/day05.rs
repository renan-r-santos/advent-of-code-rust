//! # Print Queue
//!

use std::cmp::Ordering;

use hashbrown::HashSet;

type Input = (u32, u32);

pub fn parse(input: &str) -> Input {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();

    let rules: HashSet<_> = rules_str.lines().map(|line| line.split_once("|").unwrap()).collect();

    let updates: Vec<Vec<_>> = updates_str.lines().map(|line| line.split(",").collect()).collect();

    let (mut part1, mut part2) = (0, 0);
    for update in updates {
        let mut sorted = update.clone();

        sorted.sort_unstable_by(|a, b| {
            if rules.contains(&(*b, *a)) {
                Ordering::Greater
            } else if rules.contains(&(*a, *b)) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        if update == sorted {
            part1 += sorted[update.len() / 2].parse::<u32>().unwrap();
        } else {
            part2 += sorted[update.len() / 2].parse::<u32>().unwrap();
        }
    }

    (part1, part2)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}
