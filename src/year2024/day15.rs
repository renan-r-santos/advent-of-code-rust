//! Warehouse Woes

use hashbrown::HashSet;

use crate::util::grid::*;
use crate::util::point::*;

type Input = (Grid<u8>, Vec<Point>);

pub fn parse(input: &str) -> Input {
    let (grid_str, moves_str) = input.split_once("\n\n").unwrap();

    let grid = Grid::parse(grid_str);
    let moves =
        moves_str.lines().flat_map(|line| line.chars().map(|c| Point::from(c as u8))).collect();

    (grid, moves)
}

pub fn part1((grid, moves): &(Grid<u8>, Vec<Point>)) -> i32 {
    let mut grid = grid.clone();
    let mut pos = grid.find(b'@').unwrap();

    grid[pos] = b'.';

    for &mov in moves {
        let mut peek = pos + mov;
        loop {
            match grid[peek] {
                b'#' => break,
                b'.' => {
                    pos += mov;
                    grid[pos] = b'.';
                    if pos != peek {
                        grid[peek] = b'O';
                    }
                    break;
                }
                _ => peek += mov,
            }
        }
    }
    gps_sum(&grid, b'O')
}

pub fn part2((small_grid, moves): &(Grid<u8>, Vec<Point>)) -> i32 {
    let mut grid = Grid::new(2 * small_grid.width, small_grid.height, b'X');

    for y in 0..small_grid.height {
        for x in 0..small_grid.width {
            let pos1 = Point::new(2 * x, y);
            let pos2 = Point::new(2 * x + 1, y);
            match small_grid[Point::new(x, y)] {
                b'O' => {
                    grid[pos1] = b'[';
                    grid[pos2] = b']';
                }
                b'@' => {
                    grid[pos1] = b'@';
                    grid[pos2] = b'.';
                }
                c => {
                    grid[pos1] = c;
                    grid[pos2] = c;
                }
            }
        }
    }

    let mut pos = grid.find(b'@').unwrap();
    grid[pos] = b'.';

    for &mov in moves {
        let next = pos + mov;
        match grid[next] {
            b'#' => {}
            b'.' => pos = next,
            _ => {
                if mov.x == 0 {
                    vertical_move(&mut pos, mov, &mut grid);
                } else {
                    horizontal_move(&mut pos, mov, &mut grid);
                }
            }
        }
    }
    gps_sum(&grid, b'[')
}

fn gps_sum(grid: &Grid<u8>, matcher: u8) -> i32 {
    let mut gps_sum = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid[Point::new(x, y)] == matcher {
                gps_sum += x + 100 * y;
            }
        }
    }
    gps_sum
}

fn horizontal_move(pos: &mut Point, mov: Point, grid: &mut Grid<u8>) {
    let mut peek = *pos + mov;
    loop {
        match grid[peek] {
            b'#' => break,
            b'.' => {
                while peek != *pos {
                    grid[peek] = grid[peek - mov];
                    peek -= mov;
                }
                *pos += mov;
                break;
            }
            _ => peek += mov,
        }
    }
}

fn get_counterpart(next: Point, grid: &Grid<u8>) -> Point {
    match grid[next] {
        b'[' => next + RIGHT,
        b']' => next + LEFT,
        _ => panic!("Unexpected"),
    }
}

fn vertical_move(pos: &mut Point, mov: Point, grid: &mut Grid<u8>) {
    let next = *pos + mov;
    let mut seen = vec![HashSet::from([next])];

    seen[0].insert(get_counterpart(next, grid));

    loop {
        let mut next_y = HashSet::new();
        for s in seen.last().unwrap() {
            let next = *s + mov;
            match grid[next] {
                b'#' => return,                                      // can't move
                b'.' => continue,                                    // can move, keep looking
                _c if grid[*s] == grid[next] => next_y.insert(next), // boxes are aligned
                _ => {
                    next_y.insert(next);
                    next_y.insert(get_counterpart(next, grid))
                }
            };
        }
        if next_y.is_empty() {
            for y_line in seen.into_iter().rev() {
                for half_box in y_line {
                    grid[half_box + mov] = grid[half_box];
                    grid[half_box] = b'.';
                }
            }
            *pos += mov;
            break;
        } else {
            seen.push(next_y);
        }
    }
}
