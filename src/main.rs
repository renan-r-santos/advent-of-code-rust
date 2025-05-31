use std::env::args;
use std::fs::read_to_string;
use std::iter::empty;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use aoc::util::ansi::*;
use aoc::util::parse::*;
use aoc::*;

fn main() {
    // Parse command line options
    let (year, day) = match args().nth(1) {
        Some(arg) => {
            let str = arg.as_str();
            let mut iter = str.iter_unsigned();
            (iter.next(), iter.next())
        }
        None => (None, None),
    };

    // Filter solutions
    let solutions: Vec<_> = empty()
        .chain(year2024())
        .filter(|solution| year.is_none_or(|y: u32| y == solution.year))
        .filter(|solution| day.is_none_or(|d: u32| d == solution.day))
        .collect();

    // Pretty print output for each solution.
    let mut duration = Duration::ZERO;

    for Solution { year, day, path, wrapper } in &solutions {
        if let Ok(data) = read_to_string(path) {
            let instant = Instant::now();
            let (part1, part2) = wrapper(data);
            duration += instant.elapsed();

            println!("{BOLD}{YELLOW}{year} Day {day:02}{RESET}");
            println!("    Part 1: {part1}");
            println!("    Part 2: {part2}");
        } else {
            eprintln!("{BOLD}{RED}{year} Day {day:02}{RESET}");
            eprintln!("    Missing input!");
            eprintln!("    Place input file in {BOLD}{WHITE}{}{RESET}", path.display());
        }
    }

    // Optionally print totals.
    if args().any(|a| a == "--totals") {
        println!("{BOLD}{YELLOW}⭐ {}{RESET}", 2 * solutions.len());
        println!("{BOLD}{WHITE}🕓 {} ms{RESET}", duration.as_millis());
    }
}

struct Solution {
    year: u32,
    day: u32,
    path: PathBuf,
    wrapper: fn(String) -> (String, String),
}

macro_rules! run {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$({
                let year = stringify!($year);
                let day = stringify!($day);
                let path = Path::new("input").join(year).join(day).with_extension("txt");

                let wrapper = |data: String| {
                    use $year::$day::*;

                    let input = parse(&data);
                    let part1 = part1(&input);
                    let part2 = part2(&input);

                    (part1.to_string(), part2.to_string())
                };

                Solution { year: year.unsigned(), day: day.unsigned(), path, wrapper }
            },)*]
        }
    }
}

run!(year2024
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12
);
