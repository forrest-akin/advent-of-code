use std::collections::HashMap;
use std::fs;


fn main() {
    let entries = parse_input(read_input("input"));
    let (x, y, z) = find_three_sum(2020, &entries);
    serialize_output(x, y, z);
}

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("unable to read input!")
}

fn parse_input(input: String) -> Vec<i32> {
    input.lines().map(str::parse).collect::<Result<_, _>>().unwrap()
}

fn find_three_sum(target: i32, numbers: &Vec<i32>) -> (i32, i32, i32) {
    let diff_map = key_by_diff(target, &numbers);
    let (&x, &y, &z) =
        numbers.iter()
        .filter_map(|x| {
            let y = numbers.iter().find_map(|y| diff_map.get(&(x + y)));
            if y.is_some() {
                Some((x, y.unwrap(), diff_map.get(&(x + y.unwrap())).unwrap()))
            } else { None }
        })
        .collect::<Vec<(&i32, &i32, &i32)>>()
        .first()
        .unwrap();
    (x, y, z)
}

fn key_by_diff(target: i32, numbers: &Vec<i32>) -> HashMap<i32, i32> {
    numbers.iter().map(|&x| (target - x, x)).collect()
}

fn serialize_output(x: i32, y: i32, z: i32) -> () {
    println!("{{ \"entries\": \"({}, {}, {})\", \"result\": {} }}", x, y, z, x * y * z);
}
