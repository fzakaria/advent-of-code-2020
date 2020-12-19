#![feature(str_split_once)]
#[macro_use]
extern crate lazy_static;

use advent_of_code_2020::{AdventOfCodeError, UnsolvedError};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Formatter;
use std::str::FromStr;
use std::{fmt, thread};

struct Mask {
    on: u64,
    off: u64,
}

impl Mask {
    const DO_NOTHING: Mask = Mask {
        on: 0,
        off: u64::max_value(),
    };

    // apply tne mask
    fn apply(&self, value: u64) -> u64 {
        (self.on | value) & self.off
    }

    fn new(input: &str) -> Result<Mask, AdventOfCodeError> {
        // let's calculate the on & off mask
        let mut on: u64 = 0;
        let mut off: u64 = 0;
        for (index, char) in input.chars().rev().enumerate() {
            match char {
                '0' => {
                    off = (1 << index) | off;
                }
                '1' => {
                    on = (1 << index) | on;
                }
                'X' => {
                    // do nothing
                }
                _ => {
                    return Err(AdventOfCodeError::Custom(
                        "Unmatched char for mask".to_string(),
                    ))
                }
            }
        }

        off = !off;

        Ok(Mask { on, off })
    }
}

impl fmt::Display for Mask {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "on: {:b} off: {:b}", self.on, self.off)
    }
}

fn part1(input: &str) -> Result<u64, Box<dyn Error>> {
    let mask_re = Regex::new(r"^mask\s=\s(\w+)$")?;
    let mem_re = Regex::new(r"^mem\[(\d+)]\s=\s(\d+)$")?;
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = Mask::DO_NOTHING;
    for line in input.lines() {
        if mask_re.is_match(line) {
            let captures = mask_re.captures(line).ok_or(UnsolvedError)?;
            mask = Mask::new(&captures[1])?;
            continue;
        }

        if mem_re.is_match(line) {
            let captures = mem_re.captures(line).ok_or(UnsolvedError)?;
            let address = captures[1].parse::<u64>()?;
            let value = captures[2].parse::<u64>()?;
            memory.insert(address, mask.apply(value));
            continue;
        }

        return Err(Box::new(AdventOfCodeError::Custom(
            "Unhandled regex".to_string(),
        )));
    }

    let answer: u64 = memory.values().sum();
    println!("part1: {}", answer);
    Ok(answer)
}

enum Bit {
    ZERO,
    ONE,
    FLOATING,
}

fn apply_part2_mask(mask: &str, value: u64) -> Vec<u64> {
    let mask_without_x_str = mask.replace("X", "0");
    let mask_without_x = u64::from_str_radix(&mask_without_x_str, 2).unwrap();
    let partial_mask_apply = value | mask_without_x;
    let partial_mask_apply_str = format!("{:036b}", partial_mask_apply);
    let mut mask_apply_str = String::new();
    // now go through and set matching X
    for (index, char) in partial_mask_apply_str.chars().enumerate() {
        if mask[index..index + 1] == *"X" {
            mask_apply_str.push('X');
        } else {
            mask_apply_str.push(char)
        }
    }
    let combinations = combinations(&mask_apply_str);
    combinations
        .iter()
        .map(|m| u64::from_str_radix(m, 2).unwrap())
        .collect()
}

fn combinations(input: &str) -> Vec<String> {
    if input.is_empty() {
        let mut result: Vec<String> = Vec::new();
        result.push("".to_owned());
        return result;
    }

    let letter = &input[0..1];
    let remaining = &input[1..input.len()];
    let solutions = combinations(remaining);
    let mut answer: Vec<String> = Vec::new();
    for solution in solutions {
        if letter == "0" || letter == "1" {
            answer.push("".to_string() + letter + solution.as_str());
        } else if letter == "X" {
            answer.push("".to_string() + "1" + solution.as_str());
            answer.push("".to_string() + "0" + solution.as_str());
        } else {
            panic!("Unknown letter");
        }
    }

    return answer;
}

fn part2(input: &str) -> Result<u64, Box<dyn Error>> {
    let mask_re = Regex::new(r"^mask\s=\s(\w+)$")?;
    let mem_re = Regex::new(r"^mem\[(\d+)]\s=\s(\d+)$")?;
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = "00000000000000000000000000000000000000".to_owned();
    for line in input.lines() {
        if mask_re.is_match(line) {
            let captures = mask_re.captures(line).ok_or(UnsolvedError)?;
            mask = captures[1].to_owned();
            continue;
        }

        if mem_re.is_match(line) {
            let captures = mem_re.captures(line).ok_or(UnsolvedError)?;
            let original_address = captures[1].parse::<u64>()?;
            let value = captures[2].parse::<u64>()?;

            let addresses = apply_part2_mask(&mask, original_address);
            for address in addresses {
                memory.insert(address, value);
            }
            continue;
        }

        return Err(Box::new(AdventOfCodeError::Custom(
            "Unhandled regex".to_string(),
        )));
    }

    let answer: u64 = memory.values().sum();
    println!("part2: {}", answer);

    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day14_1.txt");
    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{apply_part2_mask, part1, part2, Mask};
    use itertools::Itertools;

    #[test]
    fn mask_test() {
        let sample = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let result = Mask::new(sample);
        assert!(result.is_ok());
        let mask = result.unwrap();
        println!("{}", mask);

        assert_eq!(mask.apply(11), 73);
    }

    #[test]
    fn part1_test() {
        let sample = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[08] = 011
mem[08] = 011
mem[7] = 101
mem[8] = 0";
        let answer = part1(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 165);
    }

    fn part1_test2() {
        let sample = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let answer = part1(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 208);
    }

    #[test]
    fn part2_test() {
        let address = 42;
        let mask = "000000000000000000000000000000X1001X";
        let mut answer = apply_part2_mask(mask, address);
        answer.sort();
        assert_eq!(answer, vec![26, 27, 58, 59])
    }
}
