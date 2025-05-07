//! # Mull It Over
//!
//! Attempt at a custom parser in lieu of regex or a parser generator
use std::str::Bytes;

type Input = (u32, u32);

enum State {
    FindMul,
    FindA,
    FindB(u32),
}

#[inline]
fn to_decimal(byte: u8) -> u8 {
    byte.wrapping_sub(b'0')
}

/// Try unsigned less than 1000
fn try_unsigned(bytes: &mut Bytes<'_>) -> Option<u32> {
    let mut n = loop {
        let byte = bytes.next()?;
        let digit = to_decimal(byte);

        if digit < 10 {
            break u32::from(digit);
        }
    };

    loop {
        let Some(byte) = bytes.next() else {
            break Some(n);
        };
        let digit = to_decimal(byte);

        if digit < 10 {
            n = 10 * n + u32::from(digit);
            if n > 100 {
                break Some(n);
            }
        } else {
            break Some(n);
        }
    }
}

pub fn parse(input: &str) -> Input {
    let (mut p1, mut p2) = (0, 0);

    let mut idx = 0;
    let mut state = State::FindMul;
    let mut should_do = true;

    while idx < input.len() {
        match state {
            State::FindMul => {
                if input[idx..].starts_with("mul(") {
                    idx += 4;
                    state = State::FindA
                } else if input[idx..].starts_with("do()") {
                    should_do = true;
                    idx += 4;
                } else if input[idx..].starts_with("don't()") {
                    should_do = false;
                    idx += 7;
                } else {
                    idx += 1;
                }
            }
            State::FindA => {
                if let Some(a) = try_unsigned(&mut input[idx..].bytes()) {
                    idx += (a).ilog10() as usize + 2; // Account for the comma
                    state = State::FindB(a);
                } else {
                    idx += 1;
                    state = State::FindMul;
                }
            }
            State::FindB(a) => {
                state = State::FindMul;

                if !input[idx - 1..].starts_with(",") {
                    idx -= 1;
                    state = State::FindMul;
                    continue;
                }

                if let Some(b) = try_unsigned(&mut input[idx..].bytes()) {
                    let par_pos = idx + (b).ilog10() as usize + 1;

                    if par_pos < input.len() && input[par_pos..].starts_with(")") {
                        p1 += a * b;
                        if should_do {
                            p2 += a * b;
                        }
                        idx = par_pos + 1;
                        continue;
                    } else {
                        idx -= 1;
                    }
                }
                idx -= 1;
            }
        }
    }

    (p1, p2)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}
