use std::{fs, str::FromStr};

pub fn main() {
    let raw_input =
        fs::read_to_string("src/year_2021/day_2/input").expect("IOError: unable to read input");
    let directions = parse_input(&raw_input).unwrap();
    let solution = solve(directions);
    println!("{}", solution)
}

fn solve(directions: Vec<Direction>) -> i32 {
    directions
        .iter()
        .fold(State::default(), |state, direction| state.r#move(direction))
        .product()
}

#[derive(Default)]
struct State {
    aim: i32,
    x: i32,
    y: i32,
}

impl State {
    fn r#move(mut self, direction: &Direction) -> Self {
        match direction {
            Direction::Up(amount) => self.aim -= amount,
            Direction::Down(amount) => self.aim += amount,
            Direction::Forward(amount) => {
                self.x += amount;
                self.y += self.aim * amount;
            }
            Direction::Backward(amount) => self.x -= amount,
        };
        self
    }

    fn product(&self) -> i32 {
        self.x * self.y
    }
}

enum Direction {
    Up(i32),
    Down(i32),
    Backward(i32),
    Forward(i32),
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = s
            .split_once(' ')
            .ok_or("ParseError: input must be formatted `$DIRECTION $INT`")?;
        let amount = amount
            .parse()
            .map_err(|_| "ParseError: amount must be an integer")?;
        Ok(match direction {
            "forward" => Self::Forward(amount),
            "backward" => Self::Backward(amount),
            "up" => Self::Up(amount),
            "down" => Self::Down(amount),
            _ => return Err("ParseError: unrecognized direction"),
        })
    }
}

fn parse_input(input: &str) -> Result<Vec<Direction>, &str> {
    input.lines().map(FromStr::from_str).collect()
}
