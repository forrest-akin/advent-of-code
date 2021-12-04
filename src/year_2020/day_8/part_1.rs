use std::collections::HashSet;
use std::fs;

pub fn main() {
    let input =
        fs::read_to_string("src/year_2020/day_8/input").expect("IOError: unable to read input");
    let instructions: Vec<Instruction> = input.lines().map(parse_instruction).collect();
    let acc = Exeggutor::new(instructions)
        .last()
        .expect("No instructions processed");
    println!("{}", acc);
}

fn parse_instruction(line: &str) -> Instruction {
    let mut iter = line.split(' ');
    let operation = iter
        .next()
        .and_then(Operation::from)
        .expect("ParseError: unable to parse operation");
    let argument = iter
        .next()
        .and_then(|arg| arg.parse::<i32>().ok())
        .expect("ParseError: unable to parse argument");
    Instruction {
        operation,
        argument,
    }
}

#[derive(Debug)]
struct Exeggutor {
    instructions: Vec<Instruction>,
    program_state: ProgramState,
    history: HashSet<i32>,
}

impl Exeggutor {
    fn new(instructions: Vec<Instruction>) -> Exeggutor {
        Exeggutor {
            instructions,
            program_state: ProgramState::new(),
            history: HashSet::new(),
        }
    }
}

impl Iterator for Exeggutor {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let Exeggutor {
            instructions,
            program_state,
            history,
        } = self;
        if history.contains(&program_state.ndx) {
            None
        } else {
            instructions
                .get(program_state.ndx as usize)
                .map(|instruction| {
                    history.insert(program_state.ndx);
                    program_state.eval(instruction)
                })
        }
    }
}

#[derive(Debug)]
struct ProgramState {
    acc: i32,
    ndx: i32,
}

impl ProgramState {
    fn new() -> ProgramState {
        ProgramState { acc: 0, ndx: 0 }
    }

    fn eval(&mut self, instruction: &Instruction) -> i32 {
        match instruction.operation {
            Operation::Acc => {
                self.acc += instruction.argument;
                self.ndx += 1;
            }
            Operation::Jmp => {
                self.ndx += instruction.argument;
            }
            Operation::Nop => {
                self.ndx += 1;
            }
        };
        self.acc
    }
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    argument: i32,
}

#[derive(Debug)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

impl Operation {
    fn from(op: &str) -> Option<Operation> {
        match op {
            "acc" => Some(Operation::Acc),
            "jmp" => Some(Operation::Jmp),
            "nop" => Some(Operation::Nop),
            _ => None,
        }
    }
}
