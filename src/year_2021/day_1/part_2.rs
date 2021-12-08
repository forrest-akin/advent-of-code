use std::fs;

pub fn main() {
    let raw_input =
        fs::read_to_string("src/year_2021/day_1/input").expect("IOError: unable to read input");
    let ints = parse_input(&raw_input).unwrap();
    let count = solve(ints);
    println!("{}", count)
}

fn solve(ints: Vec<i32>) -> i32 {
    ints.windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .fold(0, |count, window| {
            if window[0] < window[1] {
                count + 1
            } else {
                count
            }
        })
}

fn parse_input(input: &str) -> Result<Vec<i32>, &str> {
    input
        .lines()
        .map(|line| {
            line.parse()
                .map_err(|_| "ParseError: each row must be an integer")
        })
        .collect()
}
