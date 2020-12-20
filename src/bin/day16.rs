#![feature(str_split_once)]
#[macro_use]
extern crate lazy_static;

use advent_of_code_2020::{AdventOfCodeError, UnsolvedError};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;
fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    Ok(0)
}

fn part2(input: &str) -> Result<u64, Box<dyn Error>> {
    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day16_1.txt");
    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};


    #[test]
    fn part1_test() {
    }

    #[test]
    fn part2_test() {}
}
