use std::collections::HashMap;
use std::fs;


fn main() {
    let entries = parse_input(read_input("input"));
    let (x, y) = find_two_sum(2020, &entries);
    println!("{}", x * y);
}

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("unable to read input!")
}

fn parse_input(input: String) -> Vec<i32> {
    input.lines().map(str::parse).collect::<Result<_, _>>().unwrap()
}

fn find_two_sum(target: i32, numbers: &Vec<i32>) -> (i32, i32) {
    let diff_map = key_by_diff(target, &numbers);
    let y = numbers.iter().find_map(|x| diff_map.get(&x)).unwrap();
    let x = diff_map.get(y).unwrap();
    (*x, *y)
}

fn key_by_diff(target: i32, numbers: &Vec<i32>) -> HashMap<i32, i32> {
    numbers.iter().map(|&x| (target - x, x)).collect()
}
