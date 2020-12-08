use std::fs;


fn main() {
    let raw_input = read_input("input").unwrap();
    let lines = parse_input(&raw_input);
    let tree_count = count_trees(&lines);
    println!("{}", tree_count)
}

fn count_trees(lines: &[&str]) -> usize {
    let row_len = lines.get(0).unwrap_or(&"").len();
    let mut indexer = (0..row_len).cycle();
    indexer.next(); // first step is from 0, not to 0

    lines.iter().skip(1)
    .filter(|line|
        next_nth(3, &mut indexer)
        .and_then(|n| line.chars().nth(n))
        .map(|c| c == '#')
        .unwrap_or(false))
    .count()
}

fn next_nth<T, U: Iterator<Item=T>>(n: i32, mut iterator: U) -> Option<T> {
    (0..n).flat_map(|_| iterator.next()).last()
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn read_input(file_path: &str) -> Result<String, &str> {
    fs::read_to_string(file_path).map_err(|_| "IO Error: unable to read input")
}
