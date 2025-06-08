//! Chronospatial Computer

use itertools::Itertools;

use crate::util::parse::*;

type Input = Cpu;

#[derive(Clone)]
pub struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Clone)]
pub struct Cpu {
    pointer: usize,
    program: Vec<u64>,
    regs: Registers,
}

impl Cpu {
    fn next(&self) -> Option<(u64, u64)> {
        if self.pointer + 1 < self.program.len() {
            let opcode = self.program[self.pointer];
            let operand = self.program[self.pointer + 1];
            Some((opcode, operand))
        } else {
            None
        }
    }

    fn combo(&self, operand: u64) -> u64 {
        match operand {
            literal @ 0..=3 => literal,
            4 => self.regs.a,
            5 => self.regs.b,
            6 => self.regs.c,
            operand => panic!("Received invalid combo operand {operand}"),
        }
    }

    fn div(&self, operand: u64) -> u64 {
        let value = self.combo(operand);
        self.regs.a / 2u64.pow(value as u32)
    }
}

pub fn parse(input: &str) -> Input {
    let mut iter = input.iter_unsigned();
    Cpu {
        pointer: 0,
        regs: Registers {
            a: iter.next().unwrap(),
            b: iter.next().unwrap(),
            c: iter.next().unwrap(),
        },
        program: iter.collect(),
    }
}

pub fn part1(input: &Input) -> String {
    let mut cpu = input.clone();
    let mut output = Vec::with_capacity(10);

    while let Some((opcode, operand)) = cpu.next() {
        match opcode {
            0 => cpu.regs.a = cpu.div(operand),
            1 => cpu.regs.b ^= operand,
            2 => cpu.regs.b = cpu.combo(operand) % 8,
            3 if cpu.regs.a != 0 => {
                cpu.pointer = operand as usize;
                continue;
            }
            4 => cpu.regs.b ^= cpu.regs.c,
            5 => output.push(cpu.combo(operand) % 8),
            6 => cpu.regs.b = cpu.div(operand),
            7 => cpu.regs.c = cpu.div(operand),
            _ => {}
        }
        cpu.pointer += 2;
    }

    output.iter().map(|o| char::from_digit(*o as u32, 10).unwrap()).join(",")
}

/// Let's analyze the input instead of trying to solve it in a generic way
/// Program:
///   B = A % 8
///   B = B XOR 5
///   C = A / (2 ^ B)
///   A = A / 8
///   B = B XOR 6
///   B = B XOR C
///   OUT B % 8
///   GOTO START IF A != 0
pub fn part2(cpu: &Input) -> u64 {
    find(0, cpu.program.len() - 1, cpu).unwrap()
}

fn get_output(a: u64) -> u64 {
    // Simplified version of the program. Given an "A", run the loop once and return the output
    let c = a >> (a & 7 ^ 5);
    (a ^ c ^ 3) & 7
}

fn find(prev_a: u64, index: usize, cpu: &Cpu) -> Option<u64> {
    let desired_out = cpu.program[index];

    (prev_a..prev_a + 8)
        .filter(|&a| get_output(a) == desired_out)
        .find_map(|a| if index == 0 { Some(a) } else { find(a * 8, index - 1, cpu) })
}
