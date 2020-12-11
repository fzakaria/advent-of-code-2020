#![feature(iterator_fold_self)]

use advent_of_code_2020::UnsolvedError;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::iter::FromIterator;
use std::ops::Deref;
use std::str::Lines;

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day8_1.txt");
    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn part2_test() {}
}
