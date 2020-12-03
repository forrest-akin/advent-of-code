use std::collections::HashMap;
use std::fs;


fn main() {
    serialize_output( find_three_sum( 2020, parse_input( read_input( "input" ) ) ) );
}

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("unable to read input!")
}

fn parse_input(input: String) -> Vec<i32> {
    input.lines().map(str::parse).collect::<Result<_, _>>().unwrap()
}

fn find_three_sum(target: i32, numbers: Vec<i32>) -> (i32, i32, i32) {
    let diff_map = key_by_diff(target, &numbers);
    numbers.iter().find_map(|&x|
        numbers.iter().find_map(|&y|
            diff_map.get(&(x + y)).map(|&z| (x, y, z))))
    .unwrap()
}

fn key_by_diff(target: i32, numbers: &Vec<i32>) -> HashMap<i32, i32> {
    numbers.iter().map(|&x| (target - x, x)).collect()
}

fn serialize_output((x, y, z): (i32, i32, i32)) {
    println!("{{ \"entries\": \"({}, {}, {})\", \"result\": {} }}", x, y, z, x * y * z);
}
