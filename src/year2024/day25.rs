//! Code Chronicle

type Input = (Vec<[usize; 5]>, Vec<[usize; 5]>);

pub fn parse(input: &str) -> Input {
    let mut locks = vec![];
    let mut keys = vec![];

    for grid in input.split("\n\n") {
        let mut sizes = [usize::MAX; 5];

        if &grid[0..5] == "#####" {
            for x in 6..11 {
                for y in 0..6 {
                    let idx = x + 6 * y;
                    if &grid[idx..idx + 1] == "." {
                        sizes[x - 6] = y;
                        break;
                    }
                }
            }
            locks.push(sizes);
        } else {
            for x in 6..11 {
                for y in 0..6 {
                    let idx = x + 6 * y;
                    if &grid[idx..idx + 1] == "#" {
                        sizes[x - 6] = 5 - y;
                        break;
                    }
                }
            }
            keys.push(sizes);
        }
    }

    (locks, keys)
}

pub fn part1((locks, keys): &Input) -> u64 {
    let mut count = 0;
    for lock in locks {
        'outer: for key in keys {
            for idx in 0..5 {
                if key[idx] + lock[idx] > 5 {
                    continue 'outer;
                }
            }
            count += 1;
        }
    }
    count
}

pub fn part2(_input: &Input) -> u64 {
    0
}
