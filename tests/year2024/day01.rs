use aoc::year2024::day01::*;

const EXAMPLE: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

const DATA: &str = include_str!("../../input/year2024/day01.txt");

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 11);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 31);
}

#[test]
fn part1_solution() {
    let input = parse(DATA);
    assert_eq!(part1(&input), 1879048);
}

#[test]
fn part2_solution() {
    let input = parse(DATA);
    assert_eq!(part2(&input), 21024792);
}
