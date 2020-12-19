#![feature(str_split_once)]
#[macro_use]
extern crate lazy_static;

use advent_of_code_2020::{AdventOfCodeError, UnsolvedError};
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;
use std::{thread, fmt};
use std::collections::HashMap;
use std::fmt::Formatter;
use regex::Regex;

struct Mask {
    on: u64,
    off: u64
}

impl Mask {

    const DO_NOTHING: Mask = Mask {
        on: 0,
        off: u64::max_value()
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
                _ => return Err(AdventOfCodeError::Custom("Unmatched char for mask".to_string()))
            }
        }

        off = !off;

        Ok(Mask {
            on,
            off
        })
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

        return Err(Box::new(AdventOfCodeError::Custom("Unhandled regex".to_string())));
    }

    let answer: u64 = memory.values().sum();
    println!("part1: {}", answer);
    Ok(answer)
}

fn part2(input: &str) -> Result<u64, Box<dyn Error>> {
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
    use crate::{part1, part2, Mask};

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
    fn part2_test() {}
}
