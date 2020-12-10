#![feature(iterator_fold_self)]

use advent_of_code_2020::UnsolvedError;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::iter::FromIterator;
use std::str::Lines;
use std::ops::Deref;

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
    return name.strip_suffix("s").unwrap_or(name)
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
                        bag: String::from(normalize_bag_name(*quantity.get(1).ok_or(UnsolvedError)?)),
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
    let mut seen : HashSet<&str> = HashSet::new();
    while !queue.is_empty() {
        // this cannot fail
        let current = queue.pop_front().ok_or(UnsolvedError)?;

        // find all my neighbors
        let neighbors : Vec<&Rule> = rules
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
