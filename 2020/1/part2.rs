use std::collections::HashMap;
use std::fs;


fn main() {
    let raw_input = read_input("input").unwrap();
    let input = parse_input(&raw_input).unwrap();
    let (x, y, z) = find_three_sum(2020, &input).expect("No triple found!");
    println!("{}", x * y * z)
}

fn find_three_sum(target: i32, numbers: &[i32]) -> Option<(i32, i32, i32)> {
    let diff_map = key_by_diff( target, &numbers );
    numbers.iter()
    .find_map(|&x|
        numbers.iter()
        .find_map(|&y| diff_map.get(&(x + y)).map(|&z| (x, y, z))))
}

fn key_by_diff(target: i32, numbers: &[i32]) -> HashMap<i32, i32> {
    numbers.iter().map( |&x| (target - x, x) ).collect()
}

fn parse_input(input: &str) -> Result<Vec<i32>, &str> {
    input.lines()
    .map(|line| line.parse().map_err(|_| "ParseError: each row must be an integer"))
    .collect()
}

fn read_input(file_path: &str) -> Result<String, &str> {
    fs::read_to_string(file_path).map_err(|_| "IOError: unable to read input")
}
