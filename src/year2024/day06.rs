//! # Guard Gallivant
//!

use hashbrown::HashSet;

type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    let mut maze: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let height = maze.len();
    let width = maze[0].len();

    let mut pos = maze
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| "<>^v".contains(c)).map(|j| (i, j)))
        .unwrap();
    let mut direction = "^>v<".chars().position(|c| c == maze[pos.0][pos.1]).unwrap();

    let get_new_pos = |p: (usize, usize), direction| -> Option<(usize, usize)> {
        match direction % 4 {
            0 => p.0.checked_sub(1).map(|row| (row, p.1)),    // Up
            1 => (p.1 + 1 < width).then_some((p.0, p.1 + 1)), // Right
            2 => (p.0 + 1 < height).then_some((p.0 + 1, p.1)), // Down
            3 => p.1.checked_sub(1).map(|col| (p.0, col)),    // Left
            _ => unreachable!(),
        }
    };

    let loop_detection = |mut pos: (usize, usize), mut direction: usize, maze: Vec<Vec<char>>| {
        let mut history: HashSet<(usize, usize, usize)> = HashSet::new();
        loop {
            history.insert((pos.0, pos.1, direction % 4));

            match get_new_pos(pos, direction) {
                Some(new_pos) if maze[new_pos.0][new_pos.1] == '#' => direction += 1,
                Some(new_pos) => {
                    if history.contains(&(new_pos.0, new_pos.1, direction % 4)) {
                        return true;
                    }
                    pos = new_pos;
                }
                None => return false,
            }
        }
    };

    maze[pos.0][pos.1] = 'X';
    loop {
        if let Some(new_pos) = get_new_pos(pos, direction) {
            if maze[new_pos.0][new_pos.1] == '#' {
                direction += 1;
            } else {
                if maze[new_pos.0][new_pos.1] == '.' {
                    let mut loop_maze = maze.clone();
                    loop_maze[new_pos.0][new_pos.1] = '#';

                    if loop_detection(pos, direction, loop_maze) {
                        maze[new_pos.0][new_pos.1] = '0';
                    }
                }
                pos = new_pos;
                if maze[pos.0][pos.1] != '0' {
                    maze[pos.0][pos.1] = 'X';
                }
            }
        } else {
            let c1 = maze.iter().flat_map(|row| row.iter()).filter(|&&c| c == 'X').count();
            let c2 = maze.iter().flat_map(|row| row.iter()).filter(|&&c| c == '0').count();
            return (c1 + c2, c2);
        }
    }
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}
