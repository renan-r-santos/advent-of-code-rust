use aoc::year2024::day16::*;

const EXAMPLE1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

const EXAMPLE2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

const DATA: &str = include_str!("../../input/year2024/day16.txt");

#[test]
fn part1_test() {
    let input = parse(EXAMPLE1);
    assert_eq!(part1(&input), 7036);

    let input = parse(EXAMPLE2);
    assert_eq!(part1(&input), 11048);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE1);
    assert_eq!(part2(&input), 45);

    let input = parse(EXAMPLE2);
    assert_eq!(part2(&input), 64);
}

#[test]
fn part1_solution() {
    let input = parse(DATA);
    assert_eq!(part1(&input), 123540);
}

#[test]
fn part2_solution() {
    let input = parse(DATA);
    assert_eq!(part2(&input), 665);
}
