use aoc::year2024::day09::*;

const EXAMPLE: &str = "2333133121414131402";

const DATA: &str = include_str!("../../input/year2024/day09.txt");

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(input), 1928);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(input), 2858);
}

#[test]
fn part1_solution() {
    let input = parse(DATA);
    assert_eq!(part1(input), 6356833654075);
}

#[test]
fn part2_solution() {
    let input = parse(DATA);
    assert_eq!(part2(input), 6389911791746);
}
