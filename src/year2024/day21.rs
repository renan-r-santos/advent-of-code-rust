//! Keypad Conundrum

use std::iter::repeat_n;

use hashbrown::HashMap;

use crate::util::grid::*;
use crate::util::point::*;

type Input = (usize, usize);
type Moves = HashMap<Vec<u8>, usize>;

const NUM_KEYPAD: &str = "789\n456\n123\n#0A";
const DIR_KEYPAD: &str = "#^A\n<v>";
const NUM_PANIC: Point = Point::new(0, 3);
const DIR_PANIC: Point = ORIGIN;

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}

pub fn parse(input: &str) -> Input {
    let num_keypad = Grid::parse(NUM_KEYPAD);
    let dir_keypad = Grid::parse(DIR_KEYPAD);

    let mut score_p1 = 0;
    let mut score_p2 = 0;

    for line in input.lines() {
        let num: usize = line[..3].parse().unwrap();

        let initial_moves = HashMap::from([(line.bytes().collect(), 1)]);
        let mut moves = apply_dirpad(initial_moves, &num_keypad, NUM_PANIC);

        for i in 0..25 {
            moves = apply_dirpad(moves, &dir_keypad, DIR_PANIC);
            let score = num * moves.iter().map(|(seq, count)| count * seq.len()).sum::<usize>();
            match i {
                1 => score_p1 += score,
                24 => score_p2 += score,
                _ => {}
            }
        }
    }

    (score_p1, score_p2)
}

fn apply_dirpad(moves: Moves, grid: &Grid<u8>, panic: Point) -> Moves {
    let mut next = HashMap::new();
    let mut src = b'A';

    for (seq, &count) in &moves {
        for &dest in seq {
            let seq = move_arm(src, dest, grid, panic);
            *next.entry(seq).or_insert(0) += count;
            src = dest;
        }
    }
    next
}

fn move_arm(src: u8, dest: u8, grid: &Grid<u8>, panic: Point) -> Vec<u8> {
    let start = grid.find(src).unwrap();
    let end = grid.find(dest).unwrap();

    let dx = end.x - start.x;
    let dy = end.y - start.y;

    let (h_char, h_count) = if dx > 0 { (b'>', dx as usize) } else { (b'<', (-dx) as usize) };
    let (v_char, v_count) = if dy > 0 { (b'v', dy as usize) } else { (b'^', (-dy) as usize) };

    let horizontal_first = {
        if dx == 0 || dy == 0 || (start.x == panic.x && end.y == panic.y) {
            true
        } else if start.y == panic.y && end.x == panic.x {
            false
        } else {
            path_score(h_char, v_char) > path_score(v_char, h_char)
        }
    };

    let mut moves = Vec::with_capacity(h_count + v_count + 1);

    if horizontal_first {
        moves.extend(repeat_n(h_char, h_count));
        moves.extend(repeat_n(v_char, v_count));
    } else {
        moves.extend(repeat_n(v_char, v_count));
        moves.extend(repeat_n(h_char, h_count));
    }
    moves.push(b'A');

    moves
}

fn path_score(first_char: u8, second_char: u8) -> i32 {
    let grid = Grid::parse(DIR_KEYPAD);

    let start_pos = grid.find(b'A').unwrap();
    let first_pos = grid.find(first_char).unwrap();
    let second_pos = grid.find(second_char).unwrap();

    start_pos.manhattan(first_pos) + first_pos.manhattan(second_pos)
}
