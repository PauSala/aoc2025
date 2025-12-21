use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

use utils::{day_input_path, read_lines};

#[derive(Debug)]
enum Operation {
    Sum,
    Multiply,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "+" => Operation::Sum,
            _ => Operation::Multiply,
        }
    }
}

pub fn part1() {
    if let Ok(lines) = read_lines(day_input_path!()) {
        let mut values: HashMap<usize, Vec<u64>> = HashMap::new();
        let mut result = 0;
        for (i, line) in lines.map_while(Result::ok).enumerate() {
            let split = line.split_whitespace();
            if i < 4 {
                for (idx, s) in split.into_iter().filter(|v| *v != " ").enumerate() {
                    let value = s.parse::<u64>().unwrap();
                    values.entry(idx).or_insert_with(Vec::new).push(value);
                }
            } else {
                for (idx, s) in split.into_iter().enumerate() {
                    if let Some(entry) = values.get(&idx) {
                        let op: Operation = s.into();
                        match op {
                            Operation::Sum => {
                                result += entry.iter().sum::<u64>();
                            }
                            Operation::Multiply => {
                                result += entry.iter().fold(1, |acc, x| acc * x);
                            }
                        }
                    }
                }
            }
        }
        println!("Result: {result}");
    }
}

pub fn part2() {
    const LINES: usize = 5;
    if let Ok(lines) = read_lines(day_input_path!()) {
        let mut values: HashMap<usize, String> = HashMap::new();
        let mut ops: VecDeque<Operation> = VecDeque::new();
        let mut result = 0;
        for (i, line) in lines.map_while(Result::ok).enumerate() {
            if i < LINES - 1 {
                for (idx, c) in line.chars().enumerate() {
                    values.entry(idx).or_insert_with(String::new).push(c);
                }
            } else {
                let split = line.split_whitespace();
                for s in split {
                    ops.push_back(s.into());
                }
            }
        }
        let mut sorted: Vec<(usize, String)> = values.into_iter().collect();
        sorted.sort();
        for (is_empty, group) in &sorted.iter().chunk_by(|(_, v)| v.trim().is_empty()) {
            if !is_empty {
                match ops.pop_front().unwrap() {
                    Operation::Sum => {
                        result += group
                            .map(|(_, v)| v.trim().parse::<u128>().unwrap())
                            .sum::<u128>()
                    }
                    Operation::Multiply => {
                        result += group
                            .map(|(_, v)| v.trim().parse::<u128>().unwrap())
                            .fold(1, |acc, x| acc * x)
                    }
                }
            }
        }
        println!("Result: {result}");
    }
}
