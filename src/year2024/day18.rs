//! RAM Run

use crate::util::grid::*;
use crate::util::heap::*;
use crate::util::parse::*;
use crate::util::point::*;

type Input = (Grid<i32>, Vec<Point>);

pub fn parse(input: &str) -> Input {
    let bytes: Vec<_> =
        input.iter_signed::<i32>().array_chunks::<2>().map(|[x, y]| Point::new(x, y)).collect();

    let mut grid = Grid::new(71, 71, i32::MAX);
    for &byte in bytes[0..1024].iter() {
        grid[byte] = i32::MIN; // Represents a corrupted location
    }

    (grid, bytes)
}

/// Uses A*
pub fn part1((grid, _): &Input) -> i32 {
    let mut grid = grid.clone();
    let mut heap = MinHeap::new();

    heap.push(0, ORIGIN);
    grid[ORIGIN] = 0;

    let end = Point::new(grid.width - 1, grid.height - 1);

    while let Some((_, pos)) = heap.pop() {
        if pos == end {
            break;
        }

        for neighbor in ORTHOGONAL.map(|dir| pos + dir) {
            if !grid.contains(neighbor) || grid[neighbor] == i32::MIN {
                continue;
            }

            let new_cost = grid[pos] + 1;
            if new_cost < grid[neighbor] {
                let priority = new_cost + neighbor.manhattan(end);
                grid[neighbor] = new_cost;
                heap.push(priority, neighbor);
            }
        }
    }
    grid[end]
}

pub fn part2((grid, bytes): &Input) -> String {
    // Let's brute force it
    let mut grid = grid.clone();
    for &byte in bytes[1024..].iter() {
        grid[byte] = i32::MIN;
        if part1(&(grid.clone(), vec![])) == i32::MAX {
            return format!("{},{}", byte.x, byte.y);
        }
    }
    unreachable!()
}
