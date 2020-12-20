#![feature(str_split_once)]
#[macro_use]
extern crate lazy_static;

use advent_of_code_2020::{AdventOfCodeError, UnsolvedError};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;
use std::thread;

fn part1(input: &str, rounds: usize) -> Result<u32, Box<dyn Error>> {
    let mut numbers: Vec<u32> = input
        .split(",")
        .map(|line| u32::from_str_radix(line, 10))
        .collect::<Result<Vec<u32>, ParseIntError>>()?;

    numbers.reserve(rounds);

    let mut history: HashMap<u32, (u32, u32)> = HashMap::with_capacity(rounds);
    // pre seed the history
    for (index, number) in numbers.iter().enumerate() {
        let round = (index + 1) as u32;
        history.insert(*number, (round, round));
    }

    // add one to the round because we do index 1
    let starting_round = numbers.len() + 1;

    for round in starting_round..=rounds {
        let previous = numbers.last().unwrap();
        // print!("round: {} previous: {}", round, previous);
        // If that was the first time the number has been spoken, the current player says 0.
        if !history.contains_key(&previous) {
            numbers.push(0);
            // first time so just insert it twice
            history.insert(0, (round as u32, round as u32));
        } else {
            let previously_spoken_rounds = history.get(&previous).unwrap();
            let difference = previously_spoken_rounds.1 - previously_spoken_rounds.0;
            numbers.push(difference);

            if !history.contains_key(&difference) {
                history.insert(difference, (round as u32, round as u32));
            } else {
                let mut previous_rounds = history.get(&difference).unwrap().clone();
                previous_rounds.0 = previous_rounds.1;
                previous_rounds.1 = round as u32;
                history.insert(difference, previous_rounds);
            }
        }

        // println!(" spoken {}", numbers.last().unwrap());
    }

    let answer = numbers.get(rounds - 1).unwrap();
    println!("part1: {}", answer);
    Ok(*answer)
}

fn part2(input: &str) -> Result<u64, Box<dyn Error>> {
    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day15_1.txt");
    part1(input, 2020)?;

    part1(input, 30_000_000)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_test_a() {
        let sample = "0,3,6";
        let answer = part1(sample, 10);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 0);
    }

    #[test]
    fn part1_test() {
        let sample = "1,3,2";
        let answer = part1(sample, 2020);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 1);
    }

    #[test]
    fn part2_test() {}
}
