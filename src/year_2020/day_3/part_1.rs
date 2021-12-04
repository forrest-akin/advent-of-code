use std::fs;


pub fn main() {
    let input = fs::read_to_string("src/year_2020/day_3/input").expect("IOError: unable to read input");
    let tree_count = count_trees(&input);
    println!("{}", tree_count)
}

fn count_trees(input: &str) -> usize {
    let row_len = input.lines().nth(0).unwrap_or(&"").len();
    let mut indexer = (0..row_len).cycle();
    indexer.next(); // first step is from 0, not to 0

    input.lines().skip(1)
    .filter(|line|
        indexer.nth(2)
        .and_then(|n| line.chars().nth(n))
        .map(|c| c == '#')
        .unwrap_or(false))
    .count()
}
