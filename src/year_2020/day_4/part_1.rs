use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs;


pub fn main() {
    let input = fs::read_to_string("src/year_2020/day_4/input").expect("IOError: unable to read input");
    let required_fields: HashSet<&str> = ["byr","iyr","eyr","hgt","hcl","ecl","pid"].iter().cloned().collect(); 
    let valid_passports =
        input.split("\n\n")
        .map(parse_passport)
        .filter(|passport|
            passport.keys().cloned()
            .collect::<HashSet<&str>>()
            .is_superset(&required_fields));

    println!("{:?}", valid_passports.count());
}

fn parse_passport(raw_passport: &str) -> HashMap<&str, &str> {
    raw_passport.split_whitespace()
    .map(|kv|
        kv.split(":")
        .collect::<Vec<&str>>()
        .try_into().expect("ParseError: key-value format should be {key}:{value}"))
    .fold(HashMap::new(), |mut map, [k, v]: [&str; 2]| {
        map.insert(k, v);
        map
    })
}
