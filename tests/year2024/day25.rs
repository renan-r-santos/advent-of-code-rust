use aoc::year2024::day25::*;

const EXAMPLE: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

const DATA: &str = include_str!("../../input/year2024/day25.txt");

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 3);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 0);
}

#[test]
fn part1_solution() {
    let input = parse(DATA);
    assert_eq!(part1(&input), 0);
}

#[test]
fn part2_solution() {
    let input = parse(DATA);
    assert_eq!(part2(&input), 0);
}
