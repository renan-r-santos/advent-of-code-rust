macro_rules! test {
    ($year:tt $($day:tt),*) => {
        pub mod $year {$(pub mod $day;)*}
    }
}

test!(year2024
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14
);
