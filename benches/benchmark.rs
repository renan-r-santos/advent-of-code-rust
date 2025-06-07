#![allow(unstable_features)]
#![feature(test)]
extern crate test;

macro_rules! benchmark {
    ($year:tt $($day:tt),*) => {
        mod $year {$(
            mod $day {
                use aoc::$year::$day::*;
                use std::fs::read_to_string;
                use std::path::Path;
                use std::sync::LazyLock;
                use test::Bencher;

                static DATA: LazyLock<String> = LazyLock::new(|| {
                    let year = stringify!($year);
                    let day = stringify!($day);
                    let path = Path::new("input").join(year).join(day).with_extension("txt");
                    read_to_string(path).unwrap()
                });

                #[bench]
                fn parse_bench(b: &mut Bencher) {
                    let data = &DATA;
                    b.iter(|| parse(data));
                }

                #[bench]
                fn part1_bench(b: &mut Bencher) {
                    let input = parse(&DATA);
                    b.iter(|| part1(&input));
                }

                #[bench]
                fn part2_bench(b: &mut Bencher) {
                    let input = parse(&DATA);
                    b.iter(|| part2(&input));
                }
            }
        )*}
    }
}

benchmark!(year2024
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15, day16
);
