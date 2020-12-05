use std::convert::TryInto;
use std::fmt::Debug;
use std::fs;
use std::io;


fn main() {
    let raw_input = read_input( "input" ).expect( "unable to read input!" );
    let passwords = parse_input( &raw_input ).expect( "unable to parse input!" );
    let valid_passwords = get_valid_passwords(&passwords);
    println!("{}", valid_passwords.len())
}

fn get_valid_passwords(passwords: &Vec<Password>) -> Vec<&Password> {
    passwords.iter()
    .filter(|&password| is_valid_password(password))
    .collect()
}

fn is_valid_password(Password { password, policy }: &Password) -> bool {
    let pattern_count = password.matches(&policy.pattern).count();
    policy.min as usize <= pattern_count
    && pattern_count <= policy.max as usize
}

fn read_input( file_path: &str ) -> Result<String, io::Error> {
    fs::read_to_string( file_path )
}

fn parse_input( input: &str ) -> Result<Vec<Password>, &str> {
    input.lines()
    .map(|line|
        line.split(": ")
        .collect::<Vec<&str>>()
        .try_into()
        .map_err(|_| "invalid format: line format must be `{policy}: {password}`")
        .and_then(|[ raw_policy, password ]: [&str; 2]|
            parse_policy( raw_policy )
            .map(|policy| Password { policy, password: password.to_string() })))
    .collect()
}

fn parse_policy( policy: &str ) -> Result<Policy, &str> {
    policy.split(" ")
    .collect::<Vec<&str>>()
    .try_into()
    .map_err(|_| "Invalid Format: policy format must be `{range} {pattern}`")
    .and_then(|[ raw_range, pattern ]: [&str; 2]|
        parse_range( raw_range )
        .map(|[min, max]| Policy { min, max, pattern: (*pattern).to_string() }))
}

fn parse_range( range: &str ) -> Result<[i32; 2], &str> {
    range.split("-")
    .map(str::parse)
    .collect::<Result<Vec<i32>, _>>()
    .map_err(|_| "Invalid Format: range min/max must be integers")
    .and_then(|vec|
        vec.try_into()
        .map_err(|_| "Invalid Format: range format must be `{min}-{max}`"))
}

#[derive(Debug)]
struct Password {
    policy: Policy,
    password: String,
}

#[derive(Debug)]
struct Policy {
    min: i32,
    max: i32,
    pattern: String,
}
