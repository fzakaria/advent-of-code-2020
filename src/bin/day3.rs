use std::error::Error;

fn part1(input: &str, right: usize, down: usize) -> Result<u64, Box<dyn Error>> {
    let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut x = 0;
    let mut y = 0;
    let mut count = 0;

    while y < board.len() {
        let curr = *board[y]
            .iter()
            .cycle()
            .nth(x)
            .expect("This should never happen");
        if curr == '#' {
            count += 1;
        }

        x += right;
        y += down;
    }

    println!("part1 count: {}", count);

    Ok(count)
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let answer = part1(input, 1, 1)?
        * part1(input, 3, 1)?
        * part1(input, 5, 1)?
        * part1(input, 7, 1)?
        * part1(input, 1, 2)?;
    println!("part2 answer: {}", answer);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day3_1.txt");
    // part 1
    part1(input, 3, 1)?;

    // part 2
    return part2(input);
}
