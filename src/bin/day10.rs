#![feature(iterator_fold_self)]

use advent_of_code_2020::UnsolvedError;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::ops::Deref;

#[derive(Debug, PartialEq, Clone)]
struct Adapter {
    jolt: i32,
}

impl Adapter {
    fn from_line(line: &str) -> Result<Adapter, UnsolvedError> {
        let jolt = i32::from_str_radix(line, 10).map_err(|_| UnsolvedError)?;
        Ok(Adapter { jolt })
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Distribution {
    one: u32,
    two: u32,
    three: u32,
}

impl Distribution {
    fn new() -> Distribution {
        Distribution {
            one: 0,
            two: 0,
            three: 0,
        }
    }
}

fn part1_greedy(adapters: &Vec<Adapter>, final_jolt: i32) -> Result<Vec<Adapter>, UnsolvedError> {
    let mut current_jolt = 0;
    let mut solution: Vec<Adapter> = Vec::new();
    let mut remaining_adapters = adapters.clone();
    while !remaining_adapters.is_empty() {
        let possible_adapters: Vec<&Adapter> = remaining_adapters
            .iter()
            .filter(|adapter| current_jolt >= adapter.jolt - 3 && current_jolt < adapter.jolt)
            .collect();

        let smallest_adapter = possible_adapters
            .into_iter()
            .min_by_key(|a| a.jolt)
            .ok_or(UnsolvedError)?
            .clone();

        current_jolt = smallest_adapter.jolt;

        remaining_adapters.remove(
            remaining_adapters
                .iter()
                .position(|x| *x == smallest_adapter)
                .unwrap(),
        );

        solution.push(smallest_adapter);
    }

    return Ok(solution);
}

fn part1(input: &str) -> Result<Distribution, Box<dyn Error>> {
    let mut adapters = input
        .lines()
        .map(|line| Adapter::from_line(line))
        .collect::<Result<Vec<Adapter>, UnsolvedError>>()?;

    let device_jolt = adapters
        .iter()
        .map(|adapter| adapter.jolt)
        .max()
        .ok_or(UnsolvedError)?
        + 3;

    // start it from the charging outlet
    // let mut seen: HashMap<String, Result<Vec<Adapter>, UnsolvedError>> = HashMap::new();
    // let answer = part1_recursion(&mut adapters, 0, device_jolt, &mut seen)?;
    let answer = part1_greedy(&adapters, device_jolt)?;

    let mut distribution = Distribution::new();
    // always add one to the one difference since we start at the 0 port
    distribution.one += 1;
    // always add one to the three difference since that's the final port
    distribution.three += 1;

    for i in 0..answer.len() - 1 {
        let current = answer.get(i).ok_or(UnsolvedError)?;
        let next = answer.get(i + 1).ok_or(UnsolvedError)?;
        let difference = next.jolt - current.jolt;
        match difference {
            1 => distribution.one += 1,
            2 => distribution.two += 1,
            3 => distribution.three += 1,
            _ => return Err(Box::new(UnsolvedError)),
        }
    }

    println!("part1: {}", distribution.one * distribution.three);

    Ok(distribution)
}

fn key(adapters: &Vec<Adapter>, starting_jolt: i32) -> String {
    let jolts = adapters.iter().map(|a| a.jolt).sorted().join(":");
    format!("{:?}_{:?}", jolts, starting_jolt)
}

fn part2_recursion(
    adapters: &Vec<Adapter>,
    starting_jolt: i32,
    seen: &mut HashMap<String, u64>,
    visited: &mut VecDeque<i32>,
) -> u64 {
    let key = key(adapters, starting_jolt);

    if starting_jolt == 0 {
        return 1;
    }

    if seen.contains_key(key.as_str()) {
        return *seen.get(key.as_str()).unwrap();
    }

    let possible_adapters: Vec<&Adapter> = adapters
        .iter()
        .filter(|adapter| adapter.jolt < starting_jolt && adapter.jolt >= starting_jolt - 3)
        .collect();

    let mut answers = 0;

    for possible_adapter in possible_adapters {
        if visited.contains(&possible_adapter.jolt) {
            continue;
        }

        visited.push_front(possible_adapter.jolt);
        answers += part2_recursion(adapters, possible_adapter.jolt, seen, visited);
        visited.pop_front();
    }

    seen.insert(key, answers);

    return answers;
}

fn part2(input: &str) -> Result<u64, Box<dyn Error>> {
    let mut adapters = input
        .lines()
        .map(|line| Adapter::from_line(line))
        .collect::<Result<Vec<Adapter>, UnsolvedError>>()?;

    // add the starting adapter
    adapters.push(Adapter { jolt: 0 });

    let final_jolt = adapters
        .iter()
        .map(|adapter| adapter.jolt)
        .max()
        .ok_or(UnsolvedError)?
        + 3;

    let mut seen = HashMap::new();
    let mut visited = VecDeque::new();
    let answer = part2_recursion(&adapters, final_jolt, &mut seen, &mut visited);

    println!("part2: {}", answer);

    Ok(answer)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day10_1.txt");
    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, Distribution};

    #[test]
    fn part1_small_test() {
        let sample = "16
10
15
5
1
11
7
19
6
12
4";
        let answer = part1(sample);
        assert!(answer.is_ok());
        assert_eq!(
            answer.unwrap(),
            Distribution {
                one: 7,
                two: 0,
                three: 5
            }
        )
    }

    #[test]
    fn part1_larger_test() {
        let sample = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let answer = part1(sample);
        assert!(answer.is_ok());
        assert_eq!(
            answer.unwrap(),
            Distribution {
                one: 22,
                two: 0,
                three: 10
            }
        )
    }

    #[test]
    fn part2_small_test() {
        let sample = "16
10
15
5
1
11
7
19
6
12
4";
        let answer = part2(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 8);
    }

    #[test]
    fn part2_larger_test() {
        let sample = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let answer = part2(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 19208);
    }
}
