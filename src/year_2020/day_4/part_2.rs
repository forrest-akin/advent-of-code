use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fs;

pub fn main() {
    let input =
        fs::read_to_string("src/year_2020/day_4/input").expect("IOError: unable to read input");
    let valid_passports = input
        .split("\n\n")
        .map(parse_passport)
        .filter(|result| result.is_ok());

    println!("{:?}", valid_passports.count());
}

fn parse_passport(raw_passport: &str) -> Result<Passport, &str> {
    let mut hash_map = raw_passport
        .split_whitespace()
        .map(|kv| {
            kv.split(':')
                .collect::<Vec<&str>>()
                .try_into()
                .expect("ParseError: key-value format should be {key}:{value}")
        })
        .fold(HashMap::new(), |mut map, [k, v]: [&str; 2]| {
            map.insert(k, v);
            map
        });
    hash_map.insert("raw_passport", raw_passport);
    PartialPassport::from(hash_map).try_into()
}

#[derive(Debug, Default)]
struct Height {
    height: i32,
    unit: String,
}

#[derive(Debug, Default)]
struct Passport {
    byr: i32,
    iyr: i32,
    eyr: i32,
    hgt: Height,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl TryFrom<PartialPassport> for Passport {
    type Error = &'static str;

    fn try_from(p: PartialPassport) -> Result<Self, Self::Error> {
        match (p.byr, p.iyr, p.eyr, p.hgt, p.hcl, p.ecl, p.pid) {
            (Some(byr), Some(iyr), Some(eyr), Some(hgt), Some(hcl), Some(ecl), Some(pid)) => {
                Ok(Passport {
                    byr,
                    iyr,
                    eyr,
                    hgt,
                    hcl,
                    ecl,
                    pid,
                    cid: p.cid,
                })
            }
            _ => Err("ParseError: missing required fields"),
        }
    }
}

#[derive(Debug, Default)]
struct PartialPassport {
    raw_passport: Option<String>,
    byr: Option<i32>,
    iyr: Option<i32>,
    eyr: Option<i32>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl From<HashMap<&str, &str>> for PartialPassport {
    fn from(hash_map: HashMap<&str, &str>) -> Self {
        PartialPassport {
            raw_passport: hash_map.get("raw_passport").map(|&x| x.to_string()),
            byr: hash_map.get("byr").and_then(|&x| parse_byr(x)),
            iyr: hash_map.get("iyr").and_then(|&x| parse_iyr(x)),
            eyr: hash_map.get("eyr").and_then(|&x| parse_eyr(x)),
            hgt: hash_map.get("hgt").and_then(|&x| parse_hgt(x)),
            hcl: hash_map.get("hcl").and_then(|&x| parse_hcl(x)),
            ecl: hash_map.get("ecl").and_then(|&x| parse_ecl(x)),
            pid: hash_map.get("pid").and_then(|&x| parse_pid(x)),
            cid: hash_map.get("cid").and_then(|&x| parse_cid(x)),
        }
    }
}

fn parse_byr(byr: &str) -> Option<i32> {
    //four digits; at least 1920 and at most 2002
    byr.parse().ok().filter(|&x| (1920..=2002).contains(&x))
}

fn parse_iyr(iyr: &str) -> Option<i32> {
    //four digits; at least 2010 and at most 2020
    iyr.parse().ok().filter(|&x| (2010..=2020).contains(&x))
}

fn parse_eyr(eyr: &str) -> Option<i32> {
    //four digits; at least 2020 and at most 2030
    eyr.parse().ok().filter(|&x| (2020..=2030).contains(&x))
}

fn parse_hgt(hgt: &str) -> Option<Height> {
    //a number followed by either cm or in:
    let (x, unit) = hgt.split_at(hgt.len() - 2);
    x.parse()
        .ok()
        .filter(|&height: &i32| match unit {
            // If cm, the number must be at least 150 and at most 193.
            "cm" => (150..=193).contains(&height),
            // If in, the number must be at least 59 and at most 76.
            "in" => (59..=76).contains(&height),
            _ => false,
        })
        .map(|height| Height {
            height,
            unit: unit.to_string(),
        })
}

fn parse_hcl(hcl: &str) -> Option<String> {
    //a # followed by exactly six characters 0-9 or a-f.
    let valid_hcls: HashSet<char> = ('0'..='9').chain('a'..='f').collect();
    let (head, tail) = hcl.split_at(1);

    Some(head)
        .filter(|&c| c == "#")
        .and(Some(tail))
        .filter(|tail| tail.len() == 6 && tail.chars().all(|c| valid_hcls.contains(&c)))
        .map(|_| hcl.to_string())
}

fn parse_ecl(ecl: &str) -> Option<String> {
    //exactly one of: amb blu brn gry grn hzl oth.
    let valid_ecls: HashSet<&str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .cloned()
        .collect();
    if valid_ecls.contains(ecl) {
        Some(ecl.to_string())
    } else {
        None
    }
}

fn parse_pid(pid: &str) -> Option<String> {
    //a nine-digit number, including leading zeroes.
    if pid.len() == 9 && pid.chars().all(char::is_numeric) {
        Some(pid.to_string())
    } else {
        None
    }
}

fn parse_cid(cid: &str) -> Option<String> {
    //ignored, missing or not.
    Some(cid.to_string())
}
