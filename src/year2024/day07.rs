//! # Bridge Repair
//!

use itertools::Itertools;

type Input = Vec<(u64, Vec<u64>)>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .map(|(result, numbers)| {
                    (
                        result.parse::<u64>().unwrap(),
                        numbers
                            .split_whitespace()
                            .map(str::parse::<u64>)
                            .map(Result::unwrap)
                            .collect(),
                    )
                })
                .unwrap()
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let ops = [|a, b| a * b, |a, b| a + b];
    let mut answer = 0;
    for (result, numbers) in input {
        let perms = (0..numbers.len() - 1)
            .map(|_| 0..ops.len())
            .multi_cartesian_product()
            .collect::<Vec<_>>();

        for perm in perms {
            let maybe_result = numbers[1..]
                .iter()
                .enumerate()
                .fold(numbers[0], |acc, (idx, x)| ops[perm[idx]](acc, *x));
            if maybe_result == *result {
                answer += result;
                break;
            }
        }
    }
    answer
}

pub fn part2(input: &Input) -> u64 {
    let ops = [|a, b| a * b, |a, b| a + b, |a, b: u64| a * 10u64.pow(b.ilog10() + 1) + b];
    let mut answer = 0;
    for (result, numbers) in input {
        let perms = (0..numbers.len() - 1)
            .map(|_| 0..ops.len())
            .multi_cartesian_product()
            .collect::<Vec<_>>();

        for perm in perms {
            let maybe_result = numbers[1..]
                .iter()
                .enumerate()
                .fold(numbers[0], |acc, (idx, x)| ops[perm[idx]](acc, *x));
            if maybe_result == *result {
                answer += result;
                break;
            }
        }
    }
    answer
}
