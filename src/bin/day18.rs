#![feature(str_split_once)]

use advent_of_code_2020::AdventOfCodeError;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Not;

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    Ok(0)
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day18_1.txt");
    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_test() {}

    #[test]
    fn part2_test() {}
}
