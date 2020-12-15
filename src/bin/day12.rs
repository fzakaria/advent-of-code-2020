use advent_of_code_2020::UnsolvedError;
use std::error::Error;

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    Ok(0)
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day12_1.txt");
    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_test() {
        assert_eq!(answer.unwrap(), 37);
    }

    #[test]
    fn part2_test() {}
}
