#![feature(iterator_fold_self)]

use advent_of_code_2020::UnsolvedError;
use std::collections::HashSet;
use std::error::Error;
use std::iter::FromIterator;
use std::str::Lines;

#[derive(Debug, PartialEq)]
struct Quantity {
    bag: String,
    amount: u32,
}

#[derive(Debug, PartialEq)]
struct Rule {
    bag: String,
    requirements: Vec<Quantity>,
}

impl Rule {
    pub fn from_line(line: &str) -> Result<Rule, UnsolvedError> {
        // first thing is remove the trailing period to make this easier
        let parts: Vec<&str> = line
            .strip_suffix(".")
            .map(|l| l.split(" contain ").collect())
            .ok_or(UnsolvedError)?;

        let name = *parts.get(0).ok_or(UnsolvedError)?;
        let remainder = *parts.get(1).ok_or(UnsolvedError)?;
        let requirements: Vec<Quantity> = match remainder {
            "no other bags" => Ok(Vec::new()),
            _ => remainder
                .split(", ")
                .map(|r| {
                    let quantity: Vec<&str> = r.splitn(2, " ").collect();
                    return Ok(Quantity {
                        bag: String::from(*quantity.get(1).ok_or(UnsolvedError)?),
                        amount: quantity
                            .get(0)
                            .and_then(|q| q.parse::<u32>().ok())
                            .ok_or(UnsolvedError)?,
                    });
                })
                .collect(),
        }?;
        Ok(Rule {
            bag: String::from(name),
            requirements,
        })
    }
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let rules: Vec<Rule> = input
        .lines()
        .map(|line| Rule::from_line(line))
        .collect::<Result<Vec<Rule>, UnsolvedError>>()?;
    for rule in rules {
        println!("{:?}", rule);
    }
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day7_1.txt");
    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {}
