use std::convert::TryInto;
use std::fmt::Debug;
use std::fs;

pub fn main() {
    let raw_input =
        fs::read_to_string("src/year_2020/day_2/input").expect("IOError: unable to read input");
    let passwords = parse_input(&raw_input).unwrap();
    let valid_passwords = get_valid_passwords(&passwords);
    println!("{}", valid_passwords.len())
}

fn get_valid_passwords(passwords: &[Password]) -> Vec<&Password> {
    passwords
        .iter()
        .filter(|password| is_valid_password(password))
        .collect()
}

fn is_valid_password(Password { password, policy }: &Password) -> bool {
    match (
        password.chars().nth(policy.min - 1),
        password.chars().nth(policy.max - 1),
    ) {
        (Some(c1), Some(c2)) => (c1 == policy.pattern || c2 == policy.pattern) && c1 != c2,
        (Some(c), _) => c == policy.pattern,
        (_, Some(c)) => c == policy.pattern,
        _ => false,
    }
}

fn parse_input(input: &str) -> Result<Vec<Password>, &str> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<Password, &str> {
    line.split(": ")
        .collect::<Vec<&str>>()
        .try_into()
        .map_err(|_| "Invalid Format: line format must be `{policy}: {password}`")
        .and_then(|[raw_policy, password]: [&str; 2]| {
            parse_policy(raw_policy).map(|policy| Password {
                policy,
                password: password.to_string(),
            })
        })
}

fn parse_policy(policy: &str) -> Result<Policy, &str> {
    policy
        .split(' ')
        .collect::<Vec<&str>>()
        .try_into()
        .map_err(|_| "Invalid Format: policy format must be `{range} {pattern}`")
        .and_then(|[raw_range, pattern]: [&str; 2]| {
            parse_range(raw_range).map(|[min, max]| Policy {
                min,
                max,
                pattern: pattern.chars().next().unwrap(),
            })
        })
}

fn parse_range(range: &str) -> Result<[usize; 2], &str> {
    range
        .split('-')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(|_| "Invalid Format: range min/max must be integers")
        .and_then(|vec: Vec<usize>| {
            vec.try_into()
                .map_err(|_| "Invalid Format: range format must be `{min}-{max}`")
        })
}

#[derive(Debug)]
struct Password {
    policy: Policy,
    password: String,
}

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    pattern: char,
}
