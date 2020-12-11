#![feature(iterator_fold_self)]

use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fmt;
use std::str::{FromStr};

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    ACC,
    JMP,
    NOP,
}

impl FromStr for Operation {
    type Err = VirtualMachineError;

    fn from_str(s: &str) -> Result<Operation, VirtualMachineError> {
        match s {
            "acc" => Ok(Operation::ACC),
            "jmp" => Ok(Operation::JMP),
            "nop" => Ok(Operation::NOP),
            _ => Err(VirtualMachineError::Custom("Unknown operation".to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Instruction {
    op: Operation,
    arg: i32,
}

impl Instruction {
    pub fn from_line(line: &str) -> Result<Instruction, VirtualMachineError> {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        Ok(Instruction {
            op: parts
                .first()
                .ok_or(VirtualMachineError::Custom("Missing operation".to_string()))
                .and_then(|op| Operation::from_str(op))?,
            arg: parts
                .get(1)
                .and_then(|arg| i32::from_str_radix(arg, 10).ok())
                .ok_or(VirtualMachineError::Custom("Could not parse argument".to_string()))?,
        })
    }
}

#[derive(Debug, PartialEq)]
struct VirtualMachine {
    acc: i64,
    pc: usize,
    instructions: Vec<Instruction>,
    seen_pc: HashSet<usize>
}

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
pub enum VirtualMachineError {
    InfiniteRecursion,
    Finished,
    Custom(String),
}

impl Error for VirtualMachineError {}

impl fmt::Display for VirtualMachineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VirtualMachineError::InfiniteRecursion => write!(f, "Infinite recursion detected."),
            VirtualMachineError::Custom(ref err) => write!(f, "Error: {}", err),
            VirtualMachineError::Finished => write!(f, "Finished."),
        }
    }
}

impl VirtualMachine {
    pub fn new(input: &str) -> Result<VirtualMachine, VirtualMachineError> {
        let instructions: Vec<Instruction> = input
            .lines()
            .map(|line| Instruction::from_line(line))
            .collect::<Result<Vec<Instruction>, VirtualMachineError>>()?;
        Ok(VirtualMachine {
            acc: 0,
            pc: 0,
            instructions,
            seen_pc: HashSet::new()
        })
    }

    pub fn next(&mut self) -> Result<(), VirtualMachineError> {
        let inst: &Instruction = self
            .instructions
            .get(self.pc)
            .ok_or(VirtualMachineError::Finished)?;

        self.seen_pc.insert(self.pc);

        match inst.op {
            Operation::ACC => {
                self.acc += inst.arg as i64;
                self.pc += 1;
            }
            Operation::JMP => {
                let next_pc = ((self.pc as i32) + inst.arg) as usize;
                if self.seen_pc.contains(&next_pc) {
                    return Err(VirtualMachineError::InfiniteRecursion);
                }
                self.pc = next_pc;
            }
            Operation::NOP => {
                self.pc += 1;
            }
        }
        Ok(())
    }
}

fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut vm = VirtualMachine::new(input)?;
    while let Ok(()) = vm.next() {
        //do nothing
    }

    println!("part1: {}", vm.acc);
    Ok(vm.acc)
}

fn jmp(vm: &mut VirtualMachine, pc: usize, inst: &Instruction, stack: &mut VecDeque<usize>, tried: bool) -> Result<i64, VirtualMachineError> {
    let next_pc = ((pc as i32) + inst.arg) as usize;
    vm.pc = next_pc;
    return solve_recursively(vm, stack, tried);
}

fn nop(vm: &mut VirtualMachine, pc: usize, stack: &mut VecDeque<usize>, tried: bool) -> Result<i64, VirtualMachineError> {
    //undo pc and just make it a no-op
    vm.pc = pc + 1;
    return solve_recursively(vm, stack, tried);
}

fn try_jmp_or_nop(vm: &mut VirtualMachine, pc: usize, acc: i64, inst: &Instruction, stack: &mut VecDeque<usize>, tried: bool) -> Result<i64, VirtualMachineError> {
    assert_eq!(inst.op, Operation::JMP);
    let result = jmp(vm, pc, inst, stack, tried);
    if result.is_ok() {
        return result;
    }
    //reset acc
    vm.acc = acc;

    return nop(vm, pc, stack, true);
}

fn try_nop_or_jmp(vm: &mut VirtualMachine, pc: usize, acc: i64, inst: &Instruction, stack: &mut VecDeque<usize>, tried: bool) -> Result<i64, VirtualMachineError> {
    assert_eq!(inst.op, Operation::NOP);
    let result = nop(vm, pc, stack, tried);
    if result.is_ok() {
        return result;
    }
    //reset acc
    vm.acc = acc;
    return jmp(vm, pc, inst, stack, true);
}

fn solve_recursively(vm: &mut VirtualMachine, stack: &mut VecDeque<usize>, tried: bool) -> Result<i64, VirtualMachineError> {
    if vm.pc == vm.instructions.len() {
        return Ok(vm.acc)
    }

    let inst: Instruction = vm.instructions
            .get(vm.pc)
            .ok_or(VirtualMachineError::Custom("Did not find nth instruction".to_string()))?.clone();

    let pc = vm.pc;

    if stack.contains(&pc) {
        return Err(VirtualMachineError::InfiniteRecursion)
    }

    stack.push_front(pc);

    let result = match inst.op {
        Operation::ACC => {
            vm.acc += inst.arg as i64;
            vm.pc += 1;
            solve_recursively(vm, stack, tried)
        }
        Operation::JMP => {
            try_jmp_or_nop(vm, pc, vm.acc, &inst, stack, tried)
        }
        Operation::NOP => {
            try_nop_or_jmp(vm, pc, vm.acc, &inst, stack, tried)
        }
    };

    stack.pop_front();

    return result;
}

fn part2(input: &str) -> Result<i64, VirtualMachineError> {
    let mut vm = VirtualMachine::new(input)?;
    let mut stack : VecDeque<usize> = VecDeque::new();
    let answer =  solve_recursively(&mut vm, &mut stack, false)?;

    println!("part2: {}", answer);
    return Ok(answer);
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day8_1.txt");
    part1(input)?;

    part2(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part2, Instruction, Operation, part1};

    #[test]
    fn instruction_convert() {
        let sample = "acc -99";
        let instruction = Instruction::from_line(sample);
        assert!(instruction.is_ok());
        assert_eq!(
            instruction.unwrap(),
            Instruction {
                op: Operation::ACC,
                arg: -99
            }
        );
    }

    #[test]
    fn part1_sample() {
        let sample = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let answer = part1(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 5);
    }

    #[test]
    fn part2_sample() {
        let sample = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let answer = part2(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 8);
    }
}
