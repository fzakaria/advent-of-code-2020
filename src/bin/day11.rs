use advent_of_code_2020::UnsolvedError;
use itertools::Itertools;
use std::error::Error;
use std::fmt;
use std::fmt::{format, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    FLOOR,
    OCCUPIED,
    EMPTY,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Status::FLOOR => write!(f, "."),
            Status::OCCUPIED => write!(f, "#"),
            Status::EMPTY => write!(f, "L"),
        }
    }
}

impl Status {
    fn from_char(c: char) -> Result<Status, UnsolvedError> {
        return match c {
            '.' => Ok(Status::FLOOR),
            'L' => Ok(Status::EMPTY),
            '#' => Ok(Status::OCCUPIED),
            _ => Err(UnsolvedError),
        };
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Position {
    row: i32,    // makes finding easier
    column: i32, // makes finding easier
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
struct Airplane {
    positions: Vec<Position>,
    rows: i32,
    columns: i32,
}

impl fmt::Display for Airplane {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for column in 0..self.columns {
                let index = (self.columns * row) + column;
                let position = self.positions.get(index as usize).unwrap();
                write!(f, "{}", position.status)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Airplane {
    fn from_input(input: &str) -> Result<Airplane, UnsolvedError> {
        let positions: Vec<Position> = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().map(move |(column, position)| {
                    Status::from_char(position).map(|status| Position {
                        row: row as i32,
                        column: column as i32,
                        status,
                    })
                })
            })
            .collect::<Result<Vec<Position>, UnsolvedError>>()?;

        // we zero index, so make sure to add one
        let rows = positions.iter().map(|p| p.row).max().ok_or(UnsolvedError)? + 1;
        let columns = positions
            .iter()
            .map(|p| p.column)
            .max()
            .ok_or(UnsolvedError)?
            + 1;

        Ok(Airplane {
            positions,
            rows,
            columns,
        })
    }

    fn key(&self) -> String {
        self.positions
            .iter()
            .map(|position| format!("{}", position.status))
            .join("")
    }

    fn right(&self, position: &Position) -> Option<&Position> {
        if position.column == (self.columns - 1) {
            return Option::None;
        }
        return self
            .positions
            .get(((self.columns * position.row) + position.column + 1) as usize);
    }

    fn left(&self, position: &Position) -> Option<&Position> {
        if position.column == 0 {
            return Option::None;
        }
        return self
            .positions
            .get(((self.columns * position.row) + position.column - 1) as usize);
    }

    fn down(&self, position: &Position) -> Option<&Position> {
        if position.row == 0 {
            return Option::None;
        }
        return self
            .positions
            .get(((self.columns * (position.row - 1)) + position.column) as usize);
    }

    fn up(&self, position: &Position) -> Option<&Position> {
        if position.row == self.rows - 1 {
            return Option::None;
        }
        return self
            .positions
            .get(((self.columns * (position.row + 1)) + position.column) as usize);
    }

    fn top_right(&self, position: &Position) -> Option<&Position> {
        if position.row == self.rows - 1 || position.column == self.columns - 1 {
            return Option::None;
        }
        return self
            .positions
            .get(((self.columns * (position.row + 1)) + position.column + 1) as usize);
    }

    fn top_left(&self, position: &Position) -> Option<&Position> {
        if position.row == self.rows - 1 || position.column == 0 {
            return Option::None;
        }
        return self
            .positions
            .get(((self.columns * (position.row + 1)) + position.column - 1) as usize);
    }

    fn bottom_right(&self, position: &Position) -> Option<&Position> {
        if position.row == 0 || position.column == self.columns - 1 {
            return Option::None;
        }
        return self
            .positions
            .get(((self.columns * (position.row - 1)) + position.column + 1) as usize);
    }

    fn bottom_left(&self, position: &Position) -> Option<&Position> {
        if position.row == 0 || position.column == 0 {
            return Option::None;
        }
        return self
            .positions
            .get(((self.columns * (position.row - 1)) + position.column - 1) as usize);
    }

    fn adjacent_seats(&self, position: &Position) -> Vec<&Position> {
        vec![
            self.right(position),
            self.left(position),
            self.down(position),
            self.up(position),
            self.top_right(position),
            self.top_left(position),
            self.bottom_right(position),
            self.bottom_left(position),
        ]
        .into_iter()
        .flat_map(|a| a)
        .collect()
    }
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut airplane = Airplane::from_input(input)?;
    println!(
        "airplane with rows: {} & columns: {}",
        airplane.rows, airplane.columns
    );
    loop {
        // helpful for debugging
        // println!("{}", airplane);
        let mut new_positions = Airplane {
            positions: vec![],
            rows: airplane.rows,
            columns: airplane.columns,
        };
        for position in &airplane.positions {
            match position.status {
                Status::FLOOR => {}
                Status::OCCUPIED => {
                    let adjacent = airplane.adjacent_seats(position);
                    let four_or_more = adjacent
                        .iter()
                        .filter(|seat| seat.status == Status::OCCUPIED)
                        .count();
                    if four_or_more >= 4 {
                        let mut new_position = position.clone();
                        new_position.status = Status::EMPTY;
                        new_positions.positions.push(new_position);
                        continue;
                    }
                }
                Status::EMPTY => {
                    let adjacent = airplane.adjacent_seats(position);
                    let any_occupied = adjacent.iter().any(|seat| seat.status == Status::OCCUPIED);
                    if !any_occupied {
                        let mut new_position = position.clone();
                        new_position.status = Status::OCCUPIED;
                        new_positions.positions.push(new_position);
                        continue;
                    }
                }
            }
            // if none of the rules hit, then add the seat as it was
            new_positions.positions.push(position.clone());
        }

        // if the iterations have stabilized
        if airplane.key() == new_positions.key() {
            break;
        }

        // move to the next iteration
        airplane = new_positions;
    }

    let answer = airplane
        .positions
        .iter()
        .filter(|position| position.status == Status::OCCUPIED)
        .count();

    println!("part1: {}", answer);

    Ok(answer)
}

fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    Err(Box::new(UnsolvedError))
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day11_1.txt");
    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_test() {
        let sample = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let answer = part1(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 37);
    }

    #[test]
    fn part2_test() {}
}
