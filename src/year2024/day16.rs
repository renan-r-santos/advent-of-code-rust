//! Reindeer Maze

use hashbrown::HashMap;

use crate::util::grid::*;
use crate::util::heap::*;
use crate::util::point::*;

type Input = (i32, i32);

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Position {
    loc: Point,
    dir: Point,
}

impl Position {
    fn new(loc: Point, dir: Point) -> Self {
        Position { loc, dir }
    }

    fn get_neighbors(&self) -> [(Self, i32); 3] {
        [
            (Position::new(self.loc + self.dir, self.dir), 1),
            (Position::new(self.loc, self.dir.clockwise()), 1000),
            (Position::new(self.loc, self.dir.counter_clockwise()), 1000),
        ]
    }

    fn get_rev_neighbors(&self) -> [(Self, i32); 3] {
        [
            (Position::new(self.loc - self.dir, self.dir), -1),
            (Position::new(self.loc, self.dir.clockwise()), -1000),
            (Position::new(self.loc, self.dir.counter_clockwise()), -1000),
        ]
    }
}

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);

    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();

    // Implements Dijkstra’s algorithm
    let start_pos = Position::new(start, RIGHT);

    let mut heap = MinHeap::new();
    heap.push(0, start_pos);

    let mut state = HashMap::with_capacity(grid.bytes.len());
    state.insert(start_pos, 0);

    while let Some((_, pos)) = heap.pop() {
        for (neighbor, mov_cost) in pos.get_neighbors() {
            if grid[neighbor.loc] == b'#' {
                continue;
            }

            let new_cost = state[&pos] + mov_cost;

            if state.get(&neighbor).is_none_or(|&neighbor_cost| new_cost < neighbor_cost) {
                heap.push(new_cost, neighbor);
                state.insert(neighbor, new_cost);
            }
        }
    }

    let mut shortest = grid.same_size_with(0);
    let cost = ORTHOGONAL.iter().map(|&dir| state[&Position::new(end, dir)]).min().unwrap();

    for dir in ORTHOGONAL {
        let end_pos = Position::new(end, dir);
        if state[&end_pos] == cost {
            reverse_dfs(end_pos, cost, start, &mut shortest, &state);
        }
    }

    (cost, shortest.bytes.iter().sum())
}

fn reverse_dfs(
    pos: Position,
    cost: i32,
    start: Point,
    shortest: &mut Grid<i32>,
    state: &HashMap<Position, i32>,
) {
    if pos.loc == start {
        shortest[start] = 1;
        return;
    }
    shortest[pos.loc] = 1;

    for (rev_neighbor, rev_cost) in pos.get_rev_neighbors() {
        if let Some(&prev_cost) = state.get(&rev_neighbor)
            && prev_cost == cost + rev_cost
        {
            reverse_dfs(rev_neighbor, prev_cost, start, shortest, state);
        }
    }
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}
