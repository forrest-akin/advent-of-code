use std::collections::HashSet;
use std::fs;

pub fn main() {
    let input =
        fs::read_to_string("src/year_2020/day_8/input").expect("IOError: unable to read input");
    let instructions: Vec<Instruction> = input.lines().map(parse_instruction).collect();
    let acc = fix_instructions(&instructions);
    println!("{}", acc);
}

fn fix_instructions(instructions: &[Instruction]) -> i32 {
    let toggle_ndxs = instructions
        .iter()
        .enumerate()
        .filter(|(_, instruction)| match instruction.operation {
            Operation::Jmp => true,
            Operation::Nop => true,
            Operation::Acc => false,
        })
        .map(|(ndx, _)| ndx);

    for ndx in toggle_ndxs {
        let (final_ndx, final_acc) = Exeggutor::new(instructions)
            .with_toggle_ndx(ndx as i32)
            .last()
            .expect("wat");
        if (final_ndx as usize) == instructions.len() {
            return final_acc;
        }
    }

    -1
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
struct Exeggutor<'a> {
    instructions: &'a [Instruction],
    program_state: ProgramState,
    history: HashSet<i32>,
    toggle_ndx: i32,
}

impl Exeggutor<'_> {
    fn new(instructions: &[Instruction]) -> Exeggutor {
        Exeggutor {
            instructions,
            program_state: ProgramState::new(),
            history: HashSet::new(),
            toggle_ndx: -1,
        }
    }

    fn with_toggle_ndx(mut self, ndx: i32) -> Self {
        self.toggle_ndx = ndx;
        self
    }
}

impl Iterator for Exeggutor<'_> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let Exeggutor {
            instructions,
            program_state,
            history,
            toggle_ndx,
        } = self;
        if history.contains(&program_state.ndx) {
            None
        } else {
            instructions
                .get(program_state.ndx as usize)
                .map(|instruction| {
                    history.insert(program_state.ndx);
                    if program_state.ndx != *toggle_ndx {
                        program_state.eval(instruction)
                    } else {
                        program_state.eval(&instruction.toggle_op())
                    }
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

    fn eval(&mut self, instruction: &Instruction) -> (i32, i32) {
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
        (self.ndx, self.acc)
    }
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    argument: i32,
}

impl Instruction {
    fn toggle_op(&self) -> Self {
        let argument = self.argument;
        let operation = match self.operation {
            Operation::Jmp => Operation::Nop,
            Operation::Nop => Operation::Jmp,
            Operation::Acc => Operation::Acc,
        };
        Instruction {
            operation,
            argument,
        }
    }
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
