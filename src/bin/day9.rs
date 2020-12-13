#![feature(iterator_fold_self)]

use advent_of_code_2020::UnsolvedError;
use itertools::Itertools;
use std::collections::VecDeque;
use std::error::Error;
use std::num::ParseIntError;

struct Cipher {
    history: VecDeque<i64>,
}

impl Cipher {
    fn new(backtrack_amount: usize) -> Cipher {
        Cipher {
            history: VecDeque::with_capacity(backtrack_amount),
        }
    }

    fn consume_number(&mut self, value: i64) {
        if self.history.len() == self.history.capacity() {
            //pop and add to mimic cycle]
            self.history.pop_front();
        }
        self.history.push_back(value);
    }

    fn is_valid(&self, value: i64) -> Result<bool, UnsolvedError> {
        if self.history.len() != self.history.capacity() {
            return Err(UnsolvedError);
        }

        for combination in self.history.iter().combinations(2) {
            // the two numbers must be different
            if combination.iter().unique().count() == 1 {
                continue;
            }
            let sum: i64 = combination.into_iter().sum();
            if sum == value {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

fn part1(input: &str, capacity: usize) -> Result<i64, Box<dyn Error>> {
    let mut cipher = Cipher::new(capacity);
    let numbers: Vec<i64> = input
        .lines()
        .map(|line| i64::from_str_radix(line, 10))
        .collect::<Result<Vec<i64>, ParseIntError>>()?;

    for value in numbers.iter().take(cipher.history.capacity()) {
        cipher.consume_number(*value);
    }

    for value in numbers.into_iter().skip(cipher.history.capacity()) {
        let is_valid = cipher.is_valid(value)?;
        if !is_valid {
            println!("part1: {}", value);
            return Ok(value);
        }
        cipher.consume_number(value);
    }

    Err(Box::new(UnsolvedError))
}

fn part2(input: &str, capacity: usize) -> Result<i64, Box<dyn Error>> {
    // this is the number to find
    let part1_answer = part1(input, capacity)?;

    let numbers: Vec<i64> = input
        .lines()
        .map(|line| i64::from_str_radix(line, 10))
        .collect::<Result<Vec<i64>, ParseIntError>>()?;

    // we want to produce all contigous subsequences
    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            let slice = &numbers[i..j];
            let sum: i64 = slice.iter().sum();
            if sum == part1_answer {
                let answer = slice.iter().max().unwrap() + slice.iter().min().unwrap();
                println!("part2: {}", answer);
                return Ok(answer);
            }
        }
    }

    Err(Box::new(UnsolvedError))
}

fn main() -> Result<(), Box<dyn Error>> {
    const MAX_CIPHER_LENGTH: usize = 25;

    let input = include_str!("day9_1.txt");
    part1(input, MAX_CIPHER_LENGTH)?;

    part2(input, MAX_CIPHER_LENGTH)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_test() {
        let sample = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let answer = part1(sample, 5);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 127);
    }

    #[test]
    fn part2_test() {
        let sample = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let answer = part2(sample, 5);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 62);
    }
}
