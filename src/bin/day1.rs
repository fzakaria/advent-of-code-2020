use std::str::FromStr;
use std::iter::Iterator;
use std::string::ToString;

#[derive(Debug, PartialEq)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
	let input = include_str!("day1.txt");
    input.lines().map( |line| line.to_string());
    println!("Hello, world! {}", input);
}
