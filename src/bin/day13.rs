#[macro_use]
extern crate lazy_static;

use advent_of_code_2020::{AdventOfCodeError, UnsolvedError};
use std::error::Error;

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day13_1.txt");

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
