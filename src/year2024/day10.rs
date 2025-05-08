//! # Hoof It
//!

use hashbrown::HashSet;

use crate::util::grid::*;
use crate::util::point::*;

type Input = (usize, usize);

fn find_next(input: &Grid<u8>, point: Point, trailends: &mut HashSet<Point>, ratings: &mut usize) {
    let next_elevation = input[point] + 1;

    for direction in ORTHOGONAL {
        let new_point = point + direction;
        if input.contains(new_point) && input[new_point] == next_elevation {
            if next_elevation == b'9' {
                *ratings += 1;
                trailends.insert(new_point);
            } else {
                find_next(input, new_point, trailends, ratings);
            }
        }
    }
}

pub fn parse(input: &str) -> Input {
    let input = Grid::parse(input);

    let mut scores = 0;
    let mut ratings = 0;

    for x in 0..input.width {
        for y in 0..input.height {
            let point = Point::new(x, y);
            if input[point] == b'0' {
                let mut trailends: HashSet<Point> = HashSet::new();
                find_next(&input, point, &mut trailends, &mut ratings);
                scores += trailends.len();
            }
        }
    }

    (scores, ratings)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}
