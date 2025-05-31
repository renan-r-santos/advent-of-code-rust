//! # Claw Contraption

use std::array;

use crate::util::parse::ParseOps;

type Input = Vec<[Equation; 2]>;

pub struct Equation {
    a: i64,
    b: i64,
    c: i64,
}

pub fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|system| {
            let mut iter = system.iter_signed();
            let parts: [i64; 6] = array::from_fn(|_| iter.next().unwrap());
            [
                Equation { a: parts[0], b: parts[2], c: parts[4] },
                Equation { a: parts[1], b: parts[3], c: parts[5] },
            ]
        })
        .collect()
}

pub fn part1(input: &Input) -> i64 {
    input.iter().fold(0, |acc, [eq1, eq2]| {
        let denominator = eq1.a * eq2.b - eq2.a * eq1.b;
        let x_numerator = eq2.b * eq1.c - eq1.b * eq2.c;
        let y_numerator = eq1.a * eq2.c - eq2.a * eq1.c;

        if x_numerator % denominator != 0 || y_numerator % denominator != 0 {
            acc
        } else {
            acc + 3 * (x_numerator / denominator) + (y_numerator / denominator)
        }
    })
}

pub fn part2(input: &Input) -> i64 {
    const F: i64 = 10000000000000;

    input.iter().fold(0, |acc, [eq1, eq2]| {
        let denominator = eq1.a * eq2.b - eq2.a * eq1.b;
        let x_numerator = eq2.b * (eq1.c + F) - eq1.b * (eq2.c + F);
        let y_numerator = eq1.a * (eq2.c + F) - eq2.a * (eq1.c + F);

        if x_numerator % denominator != 0 || y_numerator % denominator != 0 {
            acc
        } else {
            acc + 3 * (x_numerator / denominator) + (y_numerator / denominator)
        }
    })
}
