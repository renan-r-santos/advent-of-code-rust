//! # Resonant Collinearity
//!

use hashbrown::{HashMap, HashSet};

type Input = (usize, usize);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn antinodes(self, other: &Self, multiplier: i32) -> (Self, Self) {
        (
            Position {
                x: self.x + multiplier * (self.x - other.x),
                y: self.y + multiplier * (self.y - other.y),
            },
            Position {
                x: other.x + multiplier * (other.x - self.x),
                y: other.y + multiplier * (other.y - self.y),
            },
        )
    }

    fn is_oob(self, width: i32, height: i32) -> bool {
        !(0..width).contains(&self.x) || !(0..height).contains(&self.y)
    }
}

pub fn parse(input: &str) -> Input {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut frequencies: HashMap<_, HashSet<_>> = HashMap::new();
    let mut antinodes_p1 = HashSet::new();
    let mut antinodes_p2 = HashSet::new();

    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &c)| {
                (c != '.').then_some((c, Position { x: i as i32, y: j as i32 }))
            })
        })
        .for_each(|(c, current_pos)| {
            let positions = frequencies.entry(c).or_default();
            for &pos in positions.iter() {
                for multiplier in 0.. {
                    let (anti1, anti2) = current_pos.antinodes(&pos, multiplier);

                    if anti1.is_oob(width, height) && anti2.is_oob(width, height) {
                        break;
                    }

                    for anti in [anti1, anti2] {
                        if !anti.is_oob(width, height) {
                            antinodes_p2.insert(anti);
                            if multiplier == 1 {
                                antinodes_p1.insert(anti);
                            }
                        }
                    }
                }
            }

            positions.insert(current_pos);
        });

    (antinodes_p1.len(), antinodes_p2.len())
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}
