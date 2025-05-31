//! # Plutonian Pebbles
//!

use hashbrown::HashMap;

type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    let mut input: HashMap<_, _> =
        input.split_whitespace().map(|n| (n.parse::<u64>().unwrap(), 1)).collect();

    let mut blinked = HashMap::new();
    let mut p1 = 0;

    for i in 0..75 {
        if i == 25 {
            p1 = input.values().sum();
        }

        blinked.clear();

        let mut blink = |stone: u64, count: u64| {
            blinked
                .entry(stone)
                .and_modify(|blinked_count| *blinked_count += count)
                .or_insert(count);
        };

        input.into_iter().for_each(|(stone, count)| {
            if stone == 0 {
                blink(1, count);
            } else {
                let stone_log10 = stone.ilog10();
                if stone_log10 % 2 == 0 {
                    blink(stone * 2024, count);
                } else {
                    let factor = 10u64.pow(stone_log10 / 2 + 1);
                    blink(stone / factor, count);
                    blink(stone % factor, count);
                }
            }
        });

        input = blinked.clone();
    }

    (p1, input.values().sum())
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}
