#![feature(str_split_once)]

use advent_of_code_2020::AdventOfCodeError;
use regex::Regex;
use std::error::Error;
use std::num::ParseIntError;
use std::ops::RangeInclusive;

struct Rule<'a> {
    name: &'a str,
    ranges: Vec<RangeInclusive<i64>>,
}

struct Input<'a> {
    rules: Vec<Rule<'a>>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Input<'_> {
    fn from_str(input: &str) -> Result<Input, AdventOfCodeError> {
        let sections: Vec<&str> = input.split("\n\n").collect();
        if sections.len() != 3 {
            return Err(AdventOfCodeError::Custom(
                "Should have parsed three sections".to_string(),
            ));
        }

        let section_rules = sections[0];
        let rules = section_rules
            .lines()
            .map(|line| Rule::from_line(line))
            .collect::<Result<Vec<Rule>, AdventOfCodeError>>()?;

        let my_ticket_section = sections[1];
        let my_ticket_line =
            my_ticket_section
                .lines()
                .skip(1)
                .next()
                .ok_or(AdventOfCodeError::Custom(
                    "Missing my ticket line".to_string(),
                ))?;
        let my_ticket = Ticket::from_line(my_ticket_line)?;

        let nearby_tickets_section = sections[2];
        let nearby_tickets = nearby_tickets_section
            .lines()
            .skip(1)
            .map(|line| Ticket::from_line(line))
            .collect::<Result<Vec<Ticket>, AdventOfCodeError>>()?;

        Ok(Input {
            rules,
            my_ticket,
            nearby_tickets,
        })
    }
}

struct Ticket {
    values: Vec<i64>,
}

impl Ticket {
    fn from_line(line: &str) -> Result<Ticket, AdventOfCodeError> {
        let values = line
            .split(",")
            .map(|number| i64::from_str_radix(number, 10))
            .collect::<Result<Vec<i64>, ParseIntError>>()
            .map_err(|_| AdventOfCodeError::Custom("Could not parse ticket".to_string()))?;

        Ok(Ticket { values })
    }
}

impl Rule<'_> {
    fn range_from_str(value: &str) -> Result<RangeInclusive<i64>, AdventOfCodeError> {
        let numbers: Vec<i64> = value
            .split("-")
            .map(|number| i64::from_str_radix(number, 10))
            .collect::<Result<Vec<i64>, ParseIntError>>()
            .map_err(|_| AdventOfCodeError::Custom("Coul not parse range".to_string()))?;
        if numbers.len() != 2 {
            return Err(AdventOfCodeError::Custom(
                "Missing number for range".to_string(),
            ));
        }
        let start = numbers.first().unwrap();
        let end = numbers.last().unwrap();
        Ok(RangeInclusive::new(*start, *end))
    }

    fn from_line(line: &str) -> Result<Rule, AdventOfCodeError> {
        let parts: Vec<&str> = line.split(":").collect();
        let name = parts[0];
        let rules = parts[1].trim();
        let rules_regex = Regex::new(r"^(\d+-\d+) or (\d+-\d+)$")
            .map_err(|_| AdventOfCodeError::Custom("Regex failed to compile".to_string()))?;
        let captures = rules_regex
            .captures(rules)
            .ok_or(AdventOfCodeError::Custom(
                "Could not match regex".to_string(),
            ))?;

        let first_rule = Rule::range_from_str(&captures[1])?;
        let second_rule = Rule::range_from_str(&captures[2])?;

        Ok(Rule {
            name,
            ranges: vec![first_rule, second_rule],
        })
    }
}

fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let input = Input::from_str(input)?;

    // Start by determining which tickets are completely invalid;
    //these are tickets that contain values which aren't valid for any field.
    //Ignore your ticket for now.
    let mut error_rate: i64 = 0;
    for ticket in input.nearby_tickets {
        for value in ticket.values {
            let mut valid = false;
            for rule in &input.rules {
                for range in &rule.ranges {
                    if range.contains(&value) {
                        valid |= true;
                    }
                }
            }

            if !valid {
                error_rate += value;
            }
        }
    }

    println!("part1: {}", error_rate);
    Ok(error_rate)
}

fn part2(input: &str) -> Result<u64, Box<dyn Error>> {
    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day16_1.txt");
    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_test() {
        let sample = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

        let answer = part1(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 71);
    }

    #[test]
    fn part2_test() {}
}
