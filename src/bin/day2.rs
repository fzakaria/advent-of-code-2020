use advent_of_code_2020::UnsolvedError;
use regex::Regex;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct PasswordPolicy {
    min: u32,
    max: u32,
    letter: char,
}

#[derive(Debug, PartialEq)]
struct InputLine {
    policy: PasswordPolicy,
    password: String,
}

impl InputLine {
    fn is_valid_part1(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|c| *c == self.policy.letter)
            .count() as u32;
        return (self.policy.min <= count) && (count <= self.policy.max);
    }

    fn is_valid_part2(&self) -> bool {
        let lhs = self
            .password
            .chars()
            .nth((self.policy.min - 1) as usize)
            .map(|c| c == self.policy.letter)
            .unwrap_or(false);
        let rhs = self
            .password
            .chars()
            .nth((self.policy.max - 1) as usize)
            .map(|c| c == self.policy.letter)
            .unwrap_or(false);
        return lhs ^ rhs;
    }
}

impl FromStr for InputLine {
    type Err = UnsolvedError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+)-(\d+) ([[:alpha:]]): ([[:alpha:]]+)$").unwrap();
        if !re.is_match(line) {
            return Err(UnsolvedError);
        }
        let captures = re.captures(line).ok_or(UnsolvedError)?;
        let min = captures[1].parse::<u32>().map_err(|err| UnsolvedError)?;
        let max = captures[2].parse::<u32>().map_err(|err| UnsolvedError)?;
        let letter: char = captures[3].chars().next().ok_or(UnsolvedError)?;
        let password = captures[4].to_owned();
        return Ok(InputLine {
            policy: PasswordPolicy { min, max, letter },
            password,
        });
    }
}

fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    let lines: Vec<InputLine> = input
        .lines()
        .flat_map::<Result<InputLine, _>, _>(|line| InputLine::from_str(line))
        .collect();

    let count = lines.iter().filter(|line| line.is_valid_part1()).count() as u32;
    println!("part1 count: {}", count);
    Ok(count)
}

fn part2(input: &str) -> Result<u32, Box<dyn Error>> {
    let lines: Vec<InputLine> = input
        .lines()
        .flat_map::<Result<InputLine, _>, _>(|line| InputLine::from_str(line))
        .collect();

    let count = lines.iter().filter(|line| line.is_valid_part2()).count() as u32;
    println!("part2 count: {}", count);
    Ok(count)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day2_1.txt");
    // part 1
    part1(input)?;

    // part 2
    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn part2_sample() {
        let sample = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        let answer = part2(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 1);
    }
}
