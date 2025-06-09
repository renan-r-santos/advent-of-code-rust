use aoc::year2024::day18::*;

const DATA: &str = include_str!("../../input/year2024/day18.txt");

#[test]
fn part1_solution() {
    let input = parse(DATA);
    assert_eq!(part1(&input), 234);
}

#[test]
fn part2_solution() {
    let input = parse(DATA);
    assert_eq!(part2(&input), "58,19".to_string());
}
