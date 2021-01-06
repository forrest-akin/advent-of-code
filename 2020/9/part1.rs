use std::collections::HashMap;
use std::fs;


fn main() {
    let raw_input = fs::read_to_string("input").expect("IOError: unable to read input");
    let numbers = parse_input(&raw_input).unwrap();
    let outlier = find_outlier(&numbers).unwrap();
    println!("{:?}", outlier);
}

fn find_outlier(numbers: &[i64]) -> Option<i64> {
    numbers.windows(26).find_map(|window| {
        let (xs, x) = window.split_at(25);
        let target = *x.first().unwrap();
        match find_two_sum(target, xs) {
            None => Some(target),
            _ => None,
        }
    })
}

fn find_two_sum(target: i64, numbers: &[i64]) -> Option<(i64, i64)> {
    let diff_map = key_by_diff(target, &numbers);
    numbers.iter().find_map(|&x| diff_map.get(&x).map(|&y| (x, y)))
}

fn key_by_diff(target: i64, numbers: &[i64]) -> HashMap<i64, i64> {
    numbers.iter().map(|&x| (target - x, x)).collect()
}

fn parse_input(input: &str) -> Result<Vec<i64>, &str> {
    input.lines()
    .map(|line| line.parse().map_err(|_| "ParseError: each row must be an integer"))
    .collect()
}
