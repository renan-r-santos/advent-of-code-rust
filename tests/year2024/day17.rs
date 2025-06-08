use aoc::year2024::day17::*;

const EXAMPLE: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

const DATA: &str = include_str!("../../input/year2024/day17.txt");

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), "4,6,3,5,6,3,5,2,1,0");
}

#[test]
fn part2_test() {}

#[test]
fn part1_solution() {
    let input = parse(DATA);
    assert_eq!(part1(&input), "6,5,7,4,5,7,3,1,0");
}

#[test]
fn part2_solution() {
    let input = parse(DATA);
    assert_eq!(part2(&input), 105875099912602);
}
