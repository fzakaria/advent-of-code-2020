use advent_of_code_2020::UnsolvedError;
use std::error::Error;
use std::str::Lines;
use std::collections::HashSet;

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let lines: Lines = input.lines();

    let mut groups : Vec<Vec<&str>> = Vec::new();
    let mut group : Vec<&str> = Vec::new();
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
    let sum : usize = groups.iter()
        .map(|group| group.iter().flat_map(|person| person.chars()).collect::<HashSet<char>>())
        .map( |group| group.len())
        .sum();

    println!("part1 sum: {}", sum);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    unimplemented!()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day6_1.txt");
    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {}
