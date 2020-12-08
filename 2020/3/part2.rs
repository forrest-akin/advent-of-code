use std::fs;


fn main() {
    let raw_input = read_input("input").unwrap();
    let lines = parse_input(&raw_input);
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product =
        slopes.iter()
        .map(|slope| count_trees(*slope, &lines))
        .fold(1, |x, y| x * y);
    println!("{}", product)
}

fn count_trees((x, y): (usize, usize), lines: &[&str]) -> usize {
    let row_len = lines.get(0).unwrap_or(&"").len();
    let mut indexer = (0..row_len).cycle();
    indexer.next(); // first step is from 0, not to 0

    lines.iter().skip(y).step_by(y)
    .filter(|line|
        indexer.nth(x - 1)
        .and_then(|n| line.chars().nth(n))
        .map(|c| c == '#')
        .unwrap_or(false))
    .count()
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn read_input(file_path: &str) -> Result<String, &str> {
    fs::read_to_string(file_path).map_err(|_| "IO Error: unable to read input")
}
