#![feature(iterator_fold_self)]

use advent_of_code_2020::UnsolvedError;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::iter::FromIterator;
use std::ops::Deref;
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

fn normalize_bag_name(name: &str) -> &str {
    return name.strip_suffix("s").unwrap_or(name);
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
                        bag: String::from(normalize_bag_name(
                            *quantity.get(1).ok_or(UnsolvedError)?,
                        )),
                        amount: quantity
                            .get(0)
                            .and_then(|q| q.parse::<u32>().ok())
                            .ok_or(UnsolvedError)?,
                    });
                })
                .collect(),
        }?;
        Ok(Rule {
            bag: String::from(normalize_bag_name(name)), // normalize the name by removing the trailing s
            requirements,
        })
    }
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    // this is a shitty graph
    let rules: Vec<Rule> = input
        .lines()
        .map(|line| Rule::from_line(line))
        .collect::<Result<Vec<Rule>, UnsolvedError>>()?;

    let starting_node = "shiny gold bag";
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back(starting_node);
    // we walk the graph finding all nodes
    let mut seen: HashSet<&str> = HashSet::new();
    while !queue.is_empty() {
        // this cannot fail
        let current = queue.pop_front().ok_or(UnsolvedError)?;

        // find all my neighbors
        let neighbors: Vec<&Rule> = rules
            .iter()
            .filter(|rule| {
                rule.requirements
                    .iter()
                    .find(|requirement| requirement.bag == current)
                    .is_some()
            })
            .collect();

        for neighbor in neighbors {
            let bag = neighbor.bag.deref();
            seen.insert(bag);
            queue.push_back(bag);
        }
    }

    println!("part1: {}", seen.len());
    Ok(())
}

fn part2(input: &str) -> Result<u32, Box<dyn Error>> {
    // this is a shitty graph
    let rules: Vec<Rule> = input
        .lines()
        .map(|line| Rule::from_line(line))
        .collect::<Result<Vec<Rule>, UnsolvedError>>()?;

    let starting_node = "shiny gold bag";
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back(starting_node);
    // we walk the graph finding all nodes
    let mut result = 0;
    while !queue.is_empty() {
        let current = queue.pop_front().ok_or(UnsolvedError)?;
        let current_rule = rules
            .iter()
            .find(|r| r.bag == current)
            .ok_or(UnsolvedError)?;
        for requirement in &current_rule.requirements {
            result += requirement.amount;
            for i in 0..requirement.amount {
                queue.push_back(requirement.bag.deref());
            }
        }
    }

    println!("part2: {}", result);

    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day7_1.txt");
    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn part2_test() {
        let sample = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let answer = part2(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 126);
    }
}
