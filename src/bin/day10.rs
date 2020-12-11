#![feature(iterator_fold_self)]

use advent_of_code_2020::UnsolvedError;
use std::error::Error;

fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    Err(Box::new(UnsolvedError))
}

fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    Err(Box::new(UnsolvedError))
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day10_1.txt");
    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part2, part1};

    #[test]
    fn part1_test() {
    }

    #[test]
    fn part2_test() {
    }
}
