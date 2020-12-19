#![feature(str_split_once)]
#[macro_use]
extern crate lazy_static;

use advent_of_code_2020::{AdventOfCodeError, UnsolvedError};
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;
use std::thread;

fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    let split_once = input.split_once("\n").ok_or(UnsolvedError)?;
    let earliest_timestamp = u32::from_str_radix(split_once.0, 10)?;
    let bus_ids: Vec<u32> = split_once
        .1
        .split(",")
        .flat_map(|b| u32::from_str_radix(b, 10))
        .collect();

    let mut timestamp = earliest_timestamp;
    loop {
        for bus in &bus_ids {
            if timestamp % bus == 0 {
                let difference = timestamp - earliest_timestamp;
                let answer = bus * difference;
                println!("part1: {}", answer);
                return Ok(answer);
            }
        }

        timestamp += 1;
    }
}

fn part2(input: &str) -> Result<u64, Box<dyn Error>> {
    let split_once = input.split_once("\n").ok_or(UnsolvedError)?;
    let _ = u32::from_str_radix(split_once.0, 10)?;

    // 0 means ignore
    let bus_ids: Vec<u64> = split_once
        .1
        .split(",")
        .map(|b| u64::from_str_radix(b, 10).ok().unwrap_or(0))
        .collect();

    let mut timestamp = 0;
    loop {
        let mut found = true;
        for (index, bus) in bus_ids.iter().enumerate() {
            if *bus == 0 {
                continue;
            }

            let next_timestamp = timestamp + index as u64;
            if next_timestamp % bus != 0 {
                found = false;
                break;
            }
        }

        if found {
            println!("part2: {}", timestamp);
            return Ok(timestamp);
        }

        timestamp += 1;
    }
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
    fn part1_test() {
        let sample = "939
7,13,x,x,59,x,31,19";

        let answer = part1(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 295);
    }

    #[test]
    fn part2_test() {
        let sample = "939
7,13,x,x,59,x,31,19";

        let answer = part2(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 1068781);
    }

    #[test]
    fn part2_test2() {
        let sample = "939
1789,37,47,1889";

        // let answer = part2(sample, 1002161400);
        // assert!(answer.is_ok());
        // assert_eq!(answer.unwrap(), 1202161486);
    }
}
