//! Linen Layout

use hashbrown::HashMap;

type Input = (u64, u64);

fn find<'a>(patterns: &[&str], design: &'a str, memo: &mut HashMap<&'a str, u64>) -> u64 {
    if let Some(&cached) = memo.get(design) {
        return cached;
    }

    let mut matches = 0;

    for pattern in patterns {
        if let Some(stripped) = design.strip_prefix(pattern) {
            if stripped.is_empty() {
                matches += 1;
            } else {
                matches += find(patterns, stripped, memo);
            }
        }
    }

    memo.insert(design, matches);
    matches
}

pub fn parse(input: &str) -> Input {
    let (patterns_str, designs) = input.split_once("\n\n").unwrap();
    let patterns: Vec<_> = patterns_str.split(", ").collect();

    designs.lines().map(|design| find(&patterns, design, &mut HashMap::new())).fold(
        (0, 0),
        |(count, sum), matches| {
            if matches > 0 { (count + 1, sum + matches) } else { (count, sum) }
        },
    )
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}
