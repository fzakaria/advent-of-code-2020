#![feature(str_split_once)]

use advent_of_code_2020::AdventOfCodeError;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Not;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy)]
enum Status {
    Active,
    Inactive,
}

impl Not for Status {
    type Output = Self;

    fn not(self) -> Self::Output {
        use Status::*;

        match self {
            Active => Inactive,
            Inactive => Active,
        }
    }
}

impl From<char> for Status {
    fn from(c: char) -> Self {
        use Status::*;

        match c {
            '.' => Inactive,
            '#' => Active,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

fn neighbors_3d(cubes: &HashMap<Point, Status>, point: &Point) -> HashMap<Point, Status> {
    let mut neighbors: HashMap<Point, Status> = HashMap::new();
    for x_off in -1..=1 {
        for y_off in -1..=1 {
            for z_off in -1..=1 {
                let x = point.x + x_off;
                let y = point.y + y_off;
                let z = point.z + z_off;
                let neighbor_point = Point { x, y, z, w: 0 };

                // make sure we don't return ourselves
                if *point == neighbor_point {
                    continue;
                }
                match cubes.get(&neighbor_point) {
                    None => neighbors.insert(neighbor_point.clone(), Status::Inactive),
                    Some(status) => neighbors.insert(neighbor_point.clone(), status.clone()),
                };
            }
        }
    }

    assert_eq!(neighbors.len(), 26);
    return neighbors;
}

fn neighbors_4d(cubes: &HashMap<Point, Status>, point: &Point) -> HashMap<Point, Status> {
    let mut neighbors: HashMap<Point, Status> = HashMap::new();
    for x_off in -1..=1 {
        for y_off in -1..=1 {
            for z_off in -1..=1 {
                for w_off in -1..=1 {
                    let x = point.x + x_off;
                    let y = point.y + y_off;
                    let z = point.z + z_off;
                    let w = point.w + w_off;
                    let neighbor_point = Point { x, y, z, w };

                    // make sure we don't return ourselves
                    if *point == neighbor_point {
                        continue;
                    }

                    match cubes.get(&neighbor_point) {
                        None => neighbors.insert(neighbor_point.clone(), Status::Inactive),
                        Some(status) => neighbors.insert(neighbor_point.clone(), status.clone()),
                    };
                }
            }
        }
    }

    assert_eq!(neighbors.len(), 80);
    return neighbors;
}

fn cycle(
    cubes: &HashMap<Point, Status>,
    neighbors: fn(&HashMap<Point, Status>, &Point) -> HashMap<Point, Status>,
) -> HashMap<Point, Status> {
    // first lets expand to include the new neighbors
    let mut expanded_cubes = cubes.clone();
    for point in cubes.keys() {
        let neighbors = neighbors(cubes, point);
        for neighbor in neighbors.keys() {
            if !expanded_cubes.contains_key(neighbor) {
                expanded_cubes.insert(neighbor.clone(), Status::Inactive);
            }
        }
    }

    let mut next_cubes = HashMap::new();
    for (point, status) in expanded_cubes.iter() {
        let neighbors = neighbors(&expanded_cubes, point);
        let active_neighbors = neighbors
            .iter()
            .filter(|(_, &s)| s == Status::Active)
            .count();

        // insert ourselves normally
        next_cubes.insert(point.clone(), status.clone());

        // try a flip
        match status {
            Status::Active => {
                if active_neighbors != 2 && active_neighbors != 3 {
                    next_cubes.insert(point.clone(), !status.clone());
                }
            }
            Status::Inactive => {
                if active_neighbors == 3 {
                    next_cubes.insert(point.clone(), !status.clone());
                }
            }
        }
    }

    return next_cubes;
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    // parse the input
    let mut cubes = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let point = Point {
                x: x as i32,
                y: y as i32,
                z: 0,
                w: 0,
            };
            let status: Status = char.into();
            cubes.insert(point, status);
        }
    }

    for _ in 1..=6 {
        cubes = cycle(&mut cubes, neighbors_3d);
    }

    let answer = cubes
        .values()
        .filter(|&state| *state == Status::Active)
        .count();
    println!("part1: {}", answer);

    Ok(answer)
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    // parse the input
    let mut cubes = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let point = Point {
                x: x as i32,
                y: y as i32,
                z: 0,
                w: 0,
            };
            let status: Status = char.into();
            cubes.insert(point, status);
        }
    }

    for _ in 1..=6 {
        cubes = cycle(&mut cubes, neighbors_4d);
    }

    let answer = cubes
        .values()
        .filter(|&state| *state == Status::Active)
        .count();
    println!("part2: {}", answer);

    Ok(answer)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day17_1.txt");
    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_test() {
        let sample = ".#.
..#
###";
        let answer = part1(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 112);
    }

    #[test]
    fn part2_test() {}
}
