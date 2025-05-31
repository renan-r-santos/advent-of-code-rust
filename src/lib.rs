// Enable portable SIMD API: https://github.com/rust-lang/portable-simd and other
// unstable features
#![allow(unstable_features)]
#![feature(iter_array_chunks)]
#![feature(portable_simd)]

macro_rules! library {
    ($year:tt $description:literal $($day:tt),*) => {
        #[doc = concat!("# ", $description)]
        pub mod $year {$(pub mod $day;)*}
    }
}

library!(util "Utility modules to handle common recurring Advent of Code patterns."
    ansi, grid, integer, parse, point
);

library!(year2024 "Locate the Chief Historian in time for the big Christmas sleigh launch."
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12
);
