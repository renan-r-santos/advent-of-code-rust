//! Race Condition

use crate::util::grid::*;
use crate::util::point::*;

type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();

    // Find initial direction from the end position
    let mut dir = *ORTHOGONAL.iter().find(|&&mov| grid[end + mov] == b'.').unwrap();

    // Walk from end to start and check for cheats along the way
    let (mut cheats_sz2, mut cheats) = (0, 0);
    let mut map = vec![end];
    let mut pos = end;

    for steps in 0.. {
        // Find cheats
        for (index, step) in map.iter().enumerate() {
            let dist = step.manhattan(pos);
            if dist <= 20 && steps - dist - 100 >= index as i32 {
                cheats += 1;
                if dist == 2 {
                    cheats_sz2 += 1;
                }
            }
        }

        if pos == start {
            break;
        }

        // Walk to the next position
        for mov in [dir, dir.clockwise(), dir.counter_clockwise()] {
            if grid[pos + mov] != b'#' {
                dir = mov;
                pos += mov;
                map.push(pos);
                break;
            }
        }
    }

    (cheats_sz2, cheats)
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}
