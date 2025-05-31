//! # Garden Groups

use crate::util::grid::*;
use crate::util::point::*;

type Input = (u64, u64);

const TOP_LEFT: Point = Point::new(-1, -1);
const TOP_RIGHT: Point = Point::new(1, -1);
const BOTTOM_LEFT: Point = Point::new(-1, 1);
const BOTTOM_RIGHT: Point = Point::new(1, 1);

struct Region {
    area: u64,
    fences: u64,
    corners: u64,
}

struct Perimeter {
    position: Point,
    plant: u8,

    // Fences
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

impl Perimeter {
    fn count_fences(&self) -> u64 {
        self.top as u64 + self.bottom as u64 + self.left as u64 + self.right as u64
    }

    fn count_corners(&self, input: &Grid<u8>) -> u64 {
        let mut corners = 0;

        let corner_checks = [
            (self.top, self.left, TOP_LEFT),
            (self.top, self.right, TOP_RIGHT),
            (self.bottom, self.left, BOTTOM_LEFT),
            (self.bottom, self.right, BOTTOM_RIGHT),
        ];

        for (edge1, edge2, diagonal) in corner_checks {
            // Outer corner: both adjacent edges are boundaries
            if edge1 && edge2 {
                corners += 1;
            }
            // Inner corner: both adjacent edges are same plant, but diagonal is different
            else if !edge1 && !edge2 {
                let diagonal_pos = self.position + diagonal;
                if input.contains(diagonal_pos) && input[diagonal_pos] != self.plant {
                    corners += 1;
                }
            }
        }

        corners
    }
}

fn get_perimeter(position: Point, input: &Grid<u8>) -> Perimeter {
    let plant = input[position];

    Perimeter {
        position,
        plant,
        top: !input.contains(position + UP) || plant != input[position + UP],
        bottom: !input.contains(position + DOWN) || plant != input[position + DOWN],
        left: !input.contains(position + LEFT) || plant != input[position + LEFT],
        right: !input.contains(position + RIGHT) || plant != input[position + RIGHT],
    }
}

fn measure_region(
    position: Point,
    region: &mut Region,
    garden: &Grid<u8>,
    visited: &mut Grid<bool>,
) {
    let plant = garden[position];
    visited[position] = true;

    let perimeter = get_perimeter(position, garden);

    region.fences += perimeter.count_fences();
    region.corners += perimeter.count_corners(garden);
    region.area += 1;

    for direction in ORTHOGONAL {
        let next = position + direction;
        if garden.contains(next) && !visited[next] && garden[next] == plant {
            measure_region(next, region, garden, visited);
        }
    }
}

pub fn parse(input: &str) -> Input {
    let garden = Grid::parse(input);

    let (mut p1, mut p2) = (0, 0);
    let mut visited = garden.same_size_with(false);

    for x in 0..garden.width {
        for y in 0..garden.height {
            let position = Point::new(x, y);
            if !visited[position] {
                let mut region = Region { area: 0, fences: 0, corners: 0 };

                measure_region(position, &mut region, &garden, &mut visited);

                p1 += region.area * region.fences;
                p2 += region.area * region.corners;
            }
        }
    }
    (p1, p2)
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}
