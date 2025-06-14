use aoc::year2024::day20::*;

const DATA: &str = include_str!("../../input/year2024/day20.txt");

#[test]
fn part1_solution() {
    let input = parse(DATA);
    assert_eq!(part1(&input), 1530);
}

#[test]
fn part2_solution() {
    let input = parse(DATA);
    assert_eq!(part2(&input), 1033983);
}
