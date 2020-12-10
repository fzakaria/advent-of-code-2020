#![feature(iterator_fold_self)]

use advent_of_code_2020::UnsolvedError;
use std::collections::HashSet;
use std::error::Error;
use std::iter::FromIterator;
use std::str::Lines;

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let lines: Lines = input.lines();

    let mut groups: Vec<Vec<&str>> = Vec::new();
    let mut group: Vec<&str> = Vec::new();
    for line in lines {
        if line.is_empty() {
            groups.push(group.clone());
            group.clear();
            continue;
        }

        group.push(line);
    }

    if !group.is_empty() {
        groups.push(group.clone())
    }

    // now for each group turn it into a set
    let sum: usize = groups
        .iter()
        .map(|group| {
            group
                .iter()
                .flat_map(|person| person.chars())
                .collect::<HashSet<char>>()
        })
        .map(|group| group.len())
        .sum();

    println!("part1 sum: {}", sum);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let lines: Lines = input.lines();

    let mut groups: Vec<Vec<&str>> = Vec::new();
    let mut group: Vec<&str> = Vec::new();
    for line in lines {
        if line.is_empty() {
            groups.push(group.clone());
            group.clear();
            continue;
        }

        group.push(line);
    }

    if !group.is_empty() {
        groups.push(group.clone())
    }

    // now for each group turn it into a set
    let sum: usize = groups
        .iter()
        .flat_map(|group| {
            group
                .iter()
                .map(|person| person.chars())
                .map(|person| HashSet::from_iter(person))
                .fold_first(|a: HashSet<char>, b: HashSet<char>| {
                    a.intersection(&b).cloned().collect::<HashSet<char>>()
                })
        })
        .map(|group| group.len())
        .sum();

    println!("part2 sum: {}", sum);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day6_1.txt");
    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {}
