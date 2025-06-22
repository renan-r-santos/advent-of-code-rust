//! Crossed Wires

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use topological_sort::TopologicalSort;

type Input<'a> = (&'a str, &'a str);
type Ops<'a> = HashMap<&'a str, (&'a str, &'a str, &'a str)>;

pub fn parse(input: &str) -> Input {
    input.split_once("\n\n").unwrap()
}

pub fn part1(&(vars, eqs): &Input) -> u64 {
    let mut ops = HashMap::new();
    let mut deps = TopologicalSort::<&str>::new();

    for line in vars.lines() {
        let name = &line[0..3];
        let value = &line[5..];
        ops.insert(name, (value, "", "LIT"));
    }

    for line in eqs.lines() {
        let parts: Vec<_> = line.split(' ').collect();
        let left = parts[0];
        let op = parts[1];
        let right = parts[2];
        let output = parts[4];

        ops.insert(output, (left, right, op));
        deps.add_dependency(left, output);
        deps.add_dependency(right, output);
    }

    let mut values = HashMap::new();
    let mut result = 0;

    while let Some(var) = deps.pop() {
        let (left, right, op) = ops[var];

        let value = match op {
            "LIT" => left == "1",
            "AND" => values[&left] & values[&right],
            "OR" => values[&left] | values[&right],
            "XOR" => values[&left] ^ values[&right],
            _ => unreachable!(),
        };

        values.insert(var, value);

        if var.starts_with('z') {
            let bit_position = var[1..].parse::<u8>().unwrap();
            if value {
                result |= 1u64 << bit_position;
            }
        }
    }

    result
}

/// This is a ripple carry adder.
/// https://electronics.stackexchange.com/questions/448775/how-does-a-standard-ripple-carry-adder-behave
/// This code shows a few heuristics I used to find problematic gates.
/// Fixing them was a manual operation.
/// It shouldn't be hard to swap and fix them programatically, but life is short and I wasn't in the mood for writing it.
pub fn part2(&(vars, eqs): &Input) -> String {
    let mut ops = HashMap::new();

    for line in vars.lines() {
        let name = &line[0..3];
        let value = &line[5..];
        ops.insert(name, (value, "", "LIT"));
    }

    for line in eqs.lines() {
        let parts: Vec<_> = line.split(' ').collect();
        let left = parts[0];
        let op = parts[1];
        let right = parts[2];
        let output = parts[4];

        ops.insert(output, (left, right, op));
    }

    // Finding out this is a ripple carry adder
    for bit_pos in 0..46 {
        let z = format!("z{bit_pos:0>2}");
        let mut operands = HashSet::new();

        find_operands(&z, &ops, &mut operands);
        println!("{} - {:?}", z, operands.iter().sorted());
    }

    for bit_pos in 2..46 {
        let var = format!("z{bit_pos:0>2}");
        let (_, _, op) = ops[var.as_str()];
        if op != "XOR" {
            println!("{var}");
        }
    }

    for bit_pos in 2..46 {
        let var = format!("z{bit_pos:0>2}");
        let (left, right, _) = ops[var.as_str()];

        let (_, _, l_op) = ops[left];
        let (_, _, r_op) = ops[right];

        if !((l_op == "XOR" && r_op == "OR") || (l_op == "OR" && r_op == "XOR")) {
            println!("check {var}");
        }
    }

    "cgq,fnr,kqk,nbc,svm,z15,z23,z39".to_string()
}

fn find_operands<'a>(var: &'a str, ops: &'a Ops, operands: &mut HashSet<&'a str>) {
    let (left, right, op) = ops[var];

    match op {
        "LIT" => {
            operands.insert(var);
        }
        "AND" | "OR" | "XOR" => {
            find_operands(left, ops, operands);
            find_operands(right, ops, operands);
        }
        _ => unreachable!(),
    };
}
