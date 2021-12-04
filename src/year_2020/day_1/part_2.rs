use std::collections::HashMap;
use std::fs;

pub fn main() {
    let raw_input =
        fs::read_to_string("src/year_2020/day_1/input").expect("IOError: unable to read input");
    let input = parse_input(&raw_input).unwrap();
    let (x, y, z) = find_three_sum(2020, &input).expect("No triple found!");
    println!("{}", x * y * z)
}

fn find_three_sum(target: i32, numbers: &[i32]) -> Option<(i32, i32, i32)> {
    let diff_map = key_by_diff(target, numbers);
    numbers.iter().find_map(|&x| {
        numbers
            .iter()
            .find_map(|&y| diff_map.get(&(x + y)).map(|&z| (x, y, z)))
    })
}

fn key_by_diff(target: i32, numbers: &[i32]) -> HashMap<i32, i32> {
    numbers.iter().map(|&x| (target - x, x)).collect()
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
