use advent_of_code_2020::UnsolvedError;
use std::collections::HashSet;
use std::error::Error;
use std::iter::Iterator;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, PartialEq)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

fn part1() -> Result<(), Box<dyn Error>> {
    let expected_sum = 2020;
    let input = include_str!("day1_1.txt");
    let numbers: HashSet<u32> = input
        .lines()
        .flat_map::<Result<u32, _>, _>(|line| line.parse())
        .collect();
    for number in &numbers {
        let to_find = expected_sum - number;
        if numbers.contains(&to_find) {
            println!("part1: {}", to_find * number);
            return Ok(());
        }
    }

    Err(Box::new(UnsolvedError))
}

fn part2() -> Result<(), Box<dyn Error>> {
    let expected_sum = 2020;
    let input = include_str!("day1_1.txt");
    let numbers: HashSet<i32> = input
        .lines()
        .flat_map::<Result<i32, _>, _>(|line| line.parse())
        .collect();
    for number_a in &numbers {
        let to_find_first = expected_sum - number_a;
        for number_b in &numbers {
            let to_find = to_find_first - number_b;
            if numbers.contains(&to_find) {
                println!("part2: {}", to_find * number_a * number_b);
                return Ok(());
            }
        }
    }

    Err(Box::new(UnsolvedError))
}

fn main() -> Result<(), Box<dyn Error>> {
    // part 1
    part1()?;

    // part 2
    return part2();
}
