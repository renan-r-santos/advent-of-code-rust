use aoc::year2024::day22::*;

const EXAMPLE1: &str = "\
1
10
100
2024";

const EXAMPLE2: &str = "\
1
2
3
2024";

const DATA: &str = include_str!("../../input/year2024/day22.txt");

#[test]
fn part1_test() {
    let input = parse(EXAMPLE1);
    assert_eq!(part1(&input), 37327623);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE2);
    assert_eq!(part2(&input), 23);
}

#[test]
fn part1_solution() {
    let input = parse(DATA);
    assert_eq!(part1(&input), 14622549304);
}

#[test]
fn part2_solution() {
    let input = parse(DATA);
    assert_eq!(part2(&input), 1735);
}
