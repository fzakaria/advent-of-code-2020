#[macro_use]
extern crate lazy_static;

use advent_of_code_2020::{AdventOfCodeError, UnsolvedError};
use regex::Regex;
use std::error::Error;
use std::hint::unreachable_unchecked;
use std::ops::Index;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    fn from_str(input: &str) -> Result<Direction, AdventOfCodeError> {
        match input {
            "N" => Ok(Direction::NORTH),
            "E" => Ok(Direction::EAST),
            "S" => Ok(Direction::SOUTH),
            "W" => Ok(Direction::WEST),
            unknown => Err(AdventOfCodeError::Custom(format!(
                "Unknown input for direction: {}",
                unknown
            ))),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Waypoint {
    north: i32,
    east: i32,
}

impl Waypoint {

    fn rotate(&mut self, degrees: u32) {
        let times = degrees / 90;
        for _ in 0..times {
            // To rotate clockwise, replace (x,y) with (y,−x).
            let temp = self.east;
            self.east = self.north;
            self.north = temp * -1;
        }
    }

    fn advance(&mut self, direction: &Direction, units: u32) {
        match direction {
            Direction::NORTH => self.north += units as i32,
            Direction::EAST => self.east += units as i32,
            Direction::SOUTH => self.north -= units as i32,
            Direction::WEST => self.east -= units as i32
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Ship {
    north: i32,
    east: i32,
    direction: Direction,
    waypoint: Waypoint // relative to the ship's directions
}

impl Ship {
    fn new() -> Ship {
        Ship {
            north: 0,
            east: 0,
            direction: Direction::EAST,
            waypoint: Waypoint {
                north: 1,
                east: 10,
            }
        }
    }

    fn advance_to_waypoint(&mut self, units: u32) {
        self.east += (self.waypoint.east * units as i32);
        self.north += (self.waypoint.north * units as i32);
    }

    fn advance(&mut self, direction: &Direction, units: u32) {
        match direction {
            Direction::NORTH => self.north += units as i32,
            Direction::EAST => self.east += units as i32,
            Direction::SOUTH => self.north -= units as i32,
            Direction::WEST => self.east -= units as i32
        }
    }

    fn rotate(&mut self, degrees: u32) {
        let mut directions = vec![Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST];
        let index = directions.iter().position(|d| *d == self.direction).unwrap();
        directions.rotate_left(index);

        let rotations = degrees / 90;

        directions.rotate_left(rotations as usize);

        self.direction = directions.first().unwrap().clone();
    }

    fn apply(&mut self, action: &Action) {
        match action {
            Action::Movement(direction, units) => {
                self.advance(direction, *units);
            }
            Action::Rotation(degrees) => {
                self.rotate(*degrees as u32);
            },
            Action::Forward(units) => self.advance(&self.direction.clone(), *units),
        }
    }

    fn apply_part2(&mut self, action: &Action) {
        match action {
            Action::Movement(direction, units) => {
                self.waypoint.advance(direction, *units);
            }
            Action::Rotation(degrees) => {
                self.waypoint.rotate(*degrees as u32);
            },
            Action::Forward(units) => self.advance_to_waypoint(*units),
        }
    }

}

#[derive(Debug, PartialEq, Clone)]
enum Action {
    Forward(u32),
    Movement(Direction, u32),
    Rotation(u32), // it is always a clockwise rotation
}

impl Action {
    fn from_line(line: &str) -> Result<Action, AdventOfCodeError> {
        lazy_static! {
            static ref POSITION_REGEX: Regex = Regex::new(r"^(\p{Alphabetic}+)(\d+)$").unwrap();
        }
        if !POSITION_REGEX.is_match(line) {
            return Err(AdventOfCodeError::Custom("Regex did not match".to_string()));
        }

        let captures = POSITION_REGEX
            .captures(line)
            .ok_or(AdventOfCodeError::Custom(
                "Missing first capture".to_string(),
            ))?;
        let command = captures[1].to_string();
        let units = captures[2]
            .parse::<u32>()
            .map_err(|err| AdventOfCodeError::Custom("Missing second capture".to_string()))?;
        if command == "R" {
            return Ok(Action::Rotation(units));
        }

        if command == "L" {
            // always make it in terms of clockwise
            return Ok(Action::Rotation(360 - units));
        }

        if command == "F" {
            return Ok(Action::Forward(units));
        }

        let direction = Direction::from_str(command.as_str())?;

        Ok(Action::Movement(direction, units))
    }
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let actions = input
        .lines()
        .map(|line| Action::from_line(line))
        .collect::<Result<Vec<Action>, AdventOfCodeError>>()?;

    let mut ship = Ship::new();

    for action in actions {
        ship.apply(&action);
    }

    let manhattan = ship.north.abs() + ship.east.abs();
    println!("part1: {}", manhattan);
    Ok(manhattan)
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let actions = input
        .lines()
        .map(|line| Action::from_line(line))
        .collect::<Result<Vec<Action>, AdventOfCodeError>>()?;

    let mut ship = Ship::new();

    for action in actions {
        ship.apply_part2(&action);
    }

    let manhattan = ship.north.abs() + ship.east.abs();
    println!("part2: {}", manhattan);

    Ok(manhattan)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day12_1.txt");

    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, Ship, Direction};

    #[test]
    fn rotation_test() {
        let mut ship = Ship::new();
        assert_eq!(ship.direction, Direction::EAST);

        ship.rotate(90);
        assert_eq!(ship.direction, Direction::SOUTH);

        ship.rotate(180);
        assert_eq!(ship.direction, Direction::NORTH);

        ship.rotate(90);
        assert_eq!(ship.direction, Direction::EAST);
    }

    #[test]
    fn part1_test() {
        let sample = "F10
N3
F7
R90
F11";

        let answer = part1(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 25)
    }

    #[test]
    fn part2_test() {
        let sample = "F10
N3
F7
R90
F11";

        let answer = part2(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 286)

    }
}
