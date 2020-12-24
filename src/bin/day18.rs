#![feature(str_split_once)]

use crate::Operator::Plus;
use advent_of_code_2020::AdventOfCodeError;
use itertools::__std_iter::Peekable;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::num::ParseIntError;
use std::ops::Not;
use std::option::Iter;
use std::str::Chars;

/**
operator       -> "*" | "+";
group          -> "(" expression ")"
term           -> NUMBER | group;
expression     ->  term operator expression | term;
*/

enum Operator {
    Plus,
    Multiply,
}

enum Node {
    Number(u64),
    BinaryExpression {
        lhs: Box<Node>,
        op: Operator,
        rhs: Box<Node>,
    },
}

struct Parser<'a> {
    it: Peekable<Box<dyn Iterator<Item = &'a str> + 'a>>,
    next: Option<char>,
}

impl<'a> Parser<'a> {
    fn parse(value: &'a str) -> Result<Node, AdventOfCodeError> {
        let iterator: Box<dyn Iterator<Item = &str>> = Box::new(value.split_ascii_whitespace());
        let mut parser = Parser {
            it: iterator.peekable(),
            next: Option::None,
        };
        parser.expression()
    }

    /**
    * https://craftinginterpreters.com/parsing-expressions.html
    * helped me solve this one by figuring out left-associative.
    * The trick:
         We grab the matched operator token so we can track which kind of equality expression we have.
         Then we call comparison() again to parse the right-hand operand.
         We combine the operator and its two operands into a new Expr.Binary syntax tree node,
         and then loop around. Each iteration, we store the resulting expression back in the
         same expr local variable. As we zip through a sequence of equality expressions,
         that creates a left-associative nested tree of binary operator nodes.
    */
    fn expression(&mut self) -> Result<Node, AdventOfCodeError> {
        let mut expr = self.term()?;

        while self.check("+") || self.check("*") {
            let op = self.operator()?;
            let rhs = self.term()?;
            expr = Node::BinaryExpression {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            }
        }

        return Ok(expr);
    }

    fn operator(&mut self) -> Result<Operator, AdventOfCodeError> {
        let operator = self
            .consume()
            .ok_or(AdventOfCodeError::Custom("Expected operator".to_string()))?;
        return match operator {
            "+" => Ok(Operator::Plus),
            "*" => Ok(Operator::Multiply),
            _ => Err(AdventOfCodeError::Custom("Unknown operator".to_string())),
        };
    }

    fn term(&mut self) -> Result<Node, AdventOfCodeError> {
        let peek = self.peek();
        return match peek {
            None => Err(AdventOfCodeError::Custom("Expected a term".to_string())),
            Some(next) => {
                if next == "(" {
                    self.expect("(")?;
                    let group = self.expression()?;
                    self.expect(")")?;
                    Ok(group)
                } else {
                    let number = u64::from_str_radix(next, 10).map_err(|err| {
                        AdventOfCodeError::Custom(format!("failed to parse {}", next))
                    })?;
                    self.consume();
                    Ok(Node::Number(number))
                }
            }
        };
    }

    fn peek(&mut self) -> Option<&str> {
        self.it.peek().map(|&value| value)
    }

    fn consume(&mut self) -> Option<&str> {
        let consumed = self.it.next();
        consumed
    }

    fn check(&mut self, token: &str) -> bool {
        return self.it.peek().filter(|&&peek| peek == token).is_some();
    }

    fn expect(&mut self, token: &str) -> Result<(), AdventOfCodeError> {
        let next = self.peek();
        return match next {
            None => Err(AdventOfCodeError::Custom(
                "Failed to expect letter".to_string(),
            )),
            Some(letter) => {
                if letter == token {
                    self.consume();
                    Ok(())
                } else {
                    Err(AdventOfCodeError::Custom(
                        "Unexpected letter found".to_string(),
                    ))
                }
            }
        };
    }
}

fn eval(node: &Node) -> u64 {
    match node {
        Node::Number(n) => *n,
        Node::BinaryExpression { lhs, op, rhs } => {
            let lhs_val = eval(lhs);
            let rhs_val = eval(rhs);
            match op {
                Plus => lhs_val + rhs_val,
                Operator::Multiply => lhs_val * rhs_val,
            }
        }
    }
}

fn part1(input: &str) -> Result<u64, Box<dyn Error>> {
    let mut sum = 0;
    for line in input.lines() {
        let line_with_spaces = line.replace("(", "( ").replace(")", " )");
        let node = Parser::parse(&line_with_spaces)?;
        let answer = eval(&node);
        sum += answer;
    }

    println!("part1: {}", sum);
    Ok(sum)
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day18_1.txt");
    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_test() {
        let sample = "2 * 3 + (4 * 5)";
        let answer = part1(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 26);
    }

    #[test]
    fn part2_test() {}
}
