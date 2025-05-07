//! # Ceres Search
//!
type Input = Vec<Vec<char>>;

pub fn parse(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part1(input: &Input) -> u32 {
    let word = vec!['X', 'M', 'A', 'S'];
    let reversed_word = vec!['S', 'A', 'M', 'X'];

    let height = input.len();
    let width = input[0].len();

    let mut counter = 0;
    let mut tmp = vec!['.'; 4];
    for m in 0..height {
        for n in 0..width {
            // Horizontal
            if n <= width - 4 {
                tmp[0] = input[m][n];
                tmp[1] = input[m][n + 1];
                tmp[2] = input[m][n + 2];
                tmp[3] = input[m][n + 3];

                if tmp == word || tmp == reversed_word {
                    counter += 1;
                }
            }

            // Vertical
            if m <= height - 4 {
                tmp[0] = input[m][n];
                tmp[1] = input[m + 1][n];
                tmp[2] = input[m + 2][n];
                tmp[3] = input[m + 3][n];

                if tmp == word || tmp == reversed_word {
                    counter += 1;
                }
            }

            // Diagonal right
            if m <= height - 4 && n <= width - 4 {
                tmp[0] = input[m][n];
                tmp[1] = input[m + 1][n + 1];
                tmp[2] = input[m + 2][n + 2];
                tmp[3] = input[m + 3][n + 3];

                if tmp == word || tmp == reversed_word {
                    counter += 1;
                }
            }

            // Diagonal left
            if m <= height - 4 && n >= 3 {
                tmp[0] = input[m][n];
                tmp[1] = input[m + 1][n - 1];
                tmp[2] = input[m + 2][n - 2];
                tmp[3] = input[m + 3][n - 3];

                if tmp == word || tmp == reversed_word {
                    counter += 1;
                }
            }
        }
    }

    counter
}

pub fn part2(input: &Input) -> u32 {
    let word = vec!['M', 'A', 'S'];
    let reversed_word = vec!['S', 'A', 'M'];

    let height = input.len();
    let width = input[0].len();

    let mut counter = 0;

    let mut diag_right = vec!['.'; 3];
    let mut diag_left = vec!['.'; 3];

    for m in 0..height {
        for n in 0..width {
            if m <= height - 3 && n <= width - 3 {
                diag_right[0] = input[m][n];
                diag_right[1] = input[m + 1][n + 1];
                diag_right[2] = input[m + 2][n + 2];

                diag_left[0] = input[m][n + 2];
                diag_left[1] = input[m + 1][n + 1];
                diag_left[2] = input[m + 2][n];

                if (diag_right == word || diag_right == reversed_word)
                    && (diag_left == word || diag_left == reversed_word)
                {
                    counter += 1;
                }
            }
        }
    }

    counter
}
