//! Restroom Redoubt

use std::cmp::Ordering;

use crate::util::parse::*;
use crate::util::point::*;

type Input = Vec<Robot>;

pub struct Robot {
    pos: Point,
    v: Point,
}

pub fn parse(input: &str) -> Input {
    input
        .iter_signed()
        .array_chunks::<4>()
        .map(|[pos_x, pos_y, vx, vy]| Robot {
            pos: Point { x: pos_x, y: pos_y },
            v: Point { x: vx, y: vy },
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let width = 101;
    let height = 103;

    let middle_w = width / 2;
    let middle_h = height / 2;

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);

    for robot in input {
        let new_x = (robot.pos.x + 100 * robot.v.x).rem_euclid(width);
        let new_y = (robot.pos.y + 100 * robot.v.y).rem_euclid(height);

        match (new_x.cmp(&middle_w), new_y.cmp(&middle_h)) {
            (Ordering::Less, Ordering::Less) => q1 += 1,
            (Ordering::Less, Ordering::Greater) => q2 += 1,
            (Ordering::Greater, Ordering::Less) => q3 += 1,
            (Ordering::Greater, Ordering::Greater) => q4 += 1,
            _ => {} // robots in the middle are ignored
        }
    }

    q1 * q2 * q3 * q4
}

pub fn part2(_input: &Input) -> i32 {
    // No automated solution here
    0
}
