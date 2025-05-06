//! # Red-Nosed Reports
//!
use std::cmp::Ordering;

use crate::util::parse::*;

type Input = Vec<Vec<i32>>;

fn check(list: &[i32], ascending: bool) -> Option<usize> {
    for n in 1..list.len() {
        let diff = list[n] - list[n - 1];
        if (ascending && !(1..=3).contains(&diff)) || (!ascending && !(-3..=-1).contains(&diff)) {
            return Some(n);
        }
    }
    None
}

pub fn parse(input: &str) -> Input {
    input.lines().map(|line| line.iter_signed::<i32>().collect()).collect()
}

pub fn part1(input: &Input) -> u32 {
    let mut result = 0;

    'outer: for list in input {
        if list.len() < 2 {
            result += 1;
            continue;
        };

        let ascending = match (list[list.len() - 1] - list[0]).cmp(&0) {
            Ordering::Equal => continue,
            Ordering::Greater => true,
            Ordering::Less => false,
        };

        for n in 1..list.len() {
            let diff = list[n] - list[n - 1];
            if (ascending && !(1..=3).contains(&diff)) || (!ascending && !(-3..=-1).contains(&diff))
            {
                continue 'outer;
            }
        }

        result += 1;
    }
    result
}

pub fn part2(input: &Input) -> u32 {
    let mut result = 0;

    for list in input {
        if list.len() < 3 {
            result += 1;
            continue;
        };

        let ascending = match (list[list.len() - 1] - list[0]).cmp(&0) {
            Ordering::Equal => continue,
            Ordering::Greater => true,
            Ordering::Less => false,
        };

        if let Some(n) = check(list, ascending) {
            let mut list1 = list.clone();
            list1.remove(n - 1);
            let mut list2 = list.clone();
            list2.remove(n);

            if check(&list1, ascending).is_none() || check(&list2, ascending).is_none() {
                result += 1;
            }
        } else {
            result += 1;
        }
    }
    result
}
