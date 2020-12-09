use std::error::Error;
use std::str::FromStr;
use advent_of_code_2020::UnsolvedError;

#[derive(Debug, PartialEq)]
struct Seat {
    row: u32,
    column: u32,
}

impl Seat {

    pub fn seat_id(&self) -> u32 {
        (self.row * 8) + self.column
    }

    pub fn from_encoding(input: &str) -> Result<Seat, UnsolvedError> {
        // The last three characters will be either L or R; these specify exactly one of the 8 columns of seats on the plane
        let row = Seat::val_from_encoding(&input[0..input.len() - 3], "F", "B", 0, 127)?;
        let column = Seat::val_from_encoding(&input[input.len() - 3..input.len()], "L", "R", 0, 7)?;
        Ok(Seat {
            row,
            column
        })
    }

    fn val_from_encoding(input: &str, start_str: &str, end_str: &str,start: u32, end: u32) -> Result<u32, UnsolvedError> {
        if input.is_empty() {
            return Err(UnsolvedError)
        }

        // base case
        if input.len() == 1 {
            if input == start_str {
                return Ok(start)
            }
            if input == end_str {
                return Ok(end)
            }

            return Err(UnsolvedError)
        }

        let head = &input[0..1];
        let remaining = &input[1..input.len()];
        let middle = (start + end) / 2;

        if head == start_str {
            return Seat::val_from_encoding(remaining, start_str, end_str, start, middle)
        }
        if head == end_str {
            return Seat::val_from_encoding(remaining, start_str, end_str, middle + 1, end)
        }

        return Err(UnsolvedError)
    }

}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let max_seat_id = input.lines()
        .flat_map(|line| Seat::from_encoding(line))
        .map(|seat| seat.seat_id())
        .max()
        .ok_or(UnsolvedError)?;

    println!("part 1: {}", max_seat_id);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    unimplemented!()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day5_1.txt");
    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, Seat};

    #[test]
    fn seat_conversion_a() {
        let sample = "FBFBBFFRLR";
        let answer = Seat::from_encoding(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), Seat { row: 44, column: 5 });
    }

    #[test]
    fn seat_conversion_b() {
        let sample = "FFFBBBFRRR";
        let answer = Seat::from_encoding(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), Seat { row: 14, column: 7 });
    }
}
