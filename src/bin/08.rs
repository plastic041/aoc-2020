use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, Clone)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

impl Operation {
    fn from(op: &str) -> Operation {
        match op {
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            "nop" => Operation::Nop,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    operation: Operation,
    argument: i32,
}

#[derive(Debug, Clone)]
struct Program {
    accumulator: i32,
    position: usize,
    instructions: Vec<Instruction>,
}

impl Program {
    fn jmp(&mut self, argument: i32) {
        let position = self.position as i32 + argument;
        self.position = position as usize;
    }

    fn acc(&mut self, argument: i32) {
        self.accumulator = self.accumulator + argument;
        self.position += 1;
    }

    fn nop(&mut self, _argument: i32) {
        self.position += 1;
    }
}

fn parse_line(line: &str) -> Instruction {
    let mut split = line.split_whitespace();
    let op = split.next().unwrap();
    let argument = split.next().unwrap().parse().unwrap();

    Instruction {
        operation: Operation::from(op),
        argument,
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut program = Program {
        accumulator: 0,
        position: 0,
        instructions: input.lines().map(parse_line).collect_vec(),
    };

    let mut visited_positions: HashSet<usize> = HashSet::new();

    loop {
        if visited_positions.contains(&program.position) {
            break;
        } else {
            visited_positions.insert(program.position);
            let instruction = &program.instructions[program.position];
            match instruction.operation {
                Operation::Acc => program.acc(instruction.argument),
                Operation::Jmp => program.jmp(instruction.argument),
                Operation::Nop => program.nop(instruction.argument),
            }
        }
    }

    Some(program.accumulator)
}

impl Program {
    fn new(input: &str) -> Self {
        Program {
            accumulator: 0,
            position: 0,
            instructions: input.lines().map(parse_line).collect_vec(),
        }
    }
}

fn try_solve(program: &mut Program, change_position: usize) -> Option<i32> {
    let mut visited_positions: HashSet<usize> = HashSet::new();

    loop {
        if visited_positions.contains(&program.position) {
            return None;
        } else {
            visited_positions.insert(program.position);
            let mut instruction = program.instructions[program.position].clone();
            if program.position == change_position {
                match instruction.operation {
                    Operation::Acc => (),
                    Operation::Jmp => instruction.operation = Operation::Nop,
                    Operation::Nop => instruction.operation = Operation::Jmp,
                }
            }
            match instruction.operation {
                Operation::Acc => program.acc(instruction.argument),
                Operation::Jmp => program.jmp(instruction.argument),
                Operation::Nop => program.nop(instruction.argument),
            }
            if program.position == program.instructions.len() {
                return Some(program.accumulator);
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<i32> {
    let program = Program::new(input);

    for i in 0..program.instructions.len() {
        let operation = program.instructions[i].clone();
        let result = match operation.operation {
            Operation::Acc => None,
            _ => try_solve(&mut program.clone(), i),
        };

        if result.is_some() {
            return result;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
