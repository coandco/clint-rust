use std::str::FromStr;
use std::mem::discriminant;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Nop,
    Acc,
    Jmp,
}


#[derive(Debug, Clone)]
pub struct Instruction {
    opcode: Operation,
    arg: i16,
}

impl Instruction {
    // This is probably not idiomatic Rust, but I couldn't figure out how to make it work with match
    fn swap_opcode(&self, opcode1: Operation, opcode2: Operation) -> Instruction {
        if discriminant(&self.opcode) == discriminant(&opcode1) {
            Instruction { opcode: opcode2, arg: self.arg}
        } else if discriminant(&self.opcode) == discriminant(&opcode2) {
            Instruction { opcode: opcode1, arg: self.arg}
        } else {
            Instruction { opcode: self.opcode, arg: self.arg }
        }
    }
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<&str> = line.splitn(2, ' ').collect();

        match parsed.as_slice() {
            ["nop", arg] => Ok(Instruction {
                opcode: Operation::Nop,
                arg: arg
                    .parse()
                    .map_err(|_| format!("Couldn't cast arg {} to int", arg))?,
            }),
            ["acc", arg] => Ok(Instruction {
                opcode: Operation::Acc,
                arg: arg
                    .parse()
                    .map_err(|_| format!("Couldn't cast arg {} to int", arg))?,
            }),
            ["jmp", arg] => Ok(Instruction {
                opcode: Operation::Jmp,
                arg: arg
                    .parse()
                    .map_err(|_| format!("Couldn't cast arg {} to int", arg))?,
            }),
            _ => Err(format!("couldn't parse line '{}'", line))
        }
    }
}

type ProgramResult = (i32, bool);

// Adapted from https://stackoverflow.com/a/54035801
fn fancy_add(u: usize, i: &i16) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(*i as usize)
    }
}

fn run_program(data: &[Instruction]) -> ProgramResult {
    let mut acc: i32 = 0;
    let mut pc: usize = 0;
    let mut seen: HashSet<usize> = HashSet::new();
    loop {
        let current = &data[pc];
        seen.insert(pc);
        match current {
            Instruction {opcode: Operation::Acc, arg} => {
                acc += i32::from(*arg);
                pc += 1;
            },
            Instruction {opcode: Operation::Jmp, arg} => {
                pc = fancy_add(pc, arg).unwrap();
            },
            Instruction {opcode: Operation::Nop, arg: _} => {
                pc += 1;
            }
        }
        if seen.contains(&pc) {
            return (acc, true);
        }
        if pc >= data.len() {
            return (acc, false);
        }
    }
}

pub fn generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<Result<Vec<Instruction>, _>>()
        .unwrap()
}

pub fn part_one(data: &[Instruction]) -> i32 {
    let (acc, _) = run_program(data);
    acc
}

pub fn part_two(data: &[Instruction]) -> i32 {
    let mut current_swap: usize = 0;
    loop {
        let mut modified_program = data.to_owned();
        modified_program[current_swap] = modified_program[current_swap].swap_opcode(Operation::Jmp, Operation::Nop);
        let (acc, looped) = run_program(&modified_program);
        if !looped {
            return acc;
        }
        current_swap += 1;
    }
}
