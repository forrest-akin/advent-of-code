use std::fs;


fn main() {
    let raw_input = read_input("input").unwrap();
    let lines = parse_input(&raw_input);
    let tree_count = count_trees(&lines);
    println!("{}", tree_count)
}

fn count_trees(lines: &[&str]) -> usize {
    let head = lines.get(0).unwrap();
    let mut indexer = (0..head.len()).cycle();

    indexer.next();
    lines.iter().skip(1)
    .fold(0, |count, line| {
        indexer.next();
        indexer.next();
        let n = indexer.next().unwrap();
        let c = line.chars().nth(n).unwrap();
        let increment = if c == '#' { 1 } else { 0 };
        count + increment
    })
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn read_input(file_path: &str) -> Result<String, &str> {
    fs::read_to_string(file_path).map_err(|_| "IO Error: unable to read input")
}
