use aoc::year2024::day14::*;

const DATA: &str = include_str!("../../input/year2024/day14.txt");

#[test]
fn part1_solution() {
    let input = parse(DATA);
    assert_eq!(part1(&input), 228690000);
}

#[test]
fn part2_solution() {
    let input = parse(DATA);
    assert_eq!(part2(&input), 0);
}
