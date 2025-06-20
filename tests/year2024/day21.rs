use aoc::year2024::day21::*;

const EXAMPLE: &str = "\
029A
980A
179A
456A
379A";

const DATA: &str = include_str!("../../input/year2024/day21.txt");

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 126384);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 154115708116294);
}

#[test]
fn part1_solution() {
    let input = parse(DATA);
    assert_eq!(part1(&input), 248108);
}

#[test]
fn part2_solution() {
    let input = parse(DATA);
    assert_eq!(part2(&input), 303836969158972);
}
