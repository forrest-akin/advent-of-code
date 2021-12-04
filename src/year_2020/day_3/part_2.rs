use std::fs;

pub fn main() {
    let raw_input =
        fs::read_to_string("src/year_2020/day_3/input").expect("IOError: unable to read input");
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product: usize = slopes
        .iter()
        .map(|slope| count_trees(*slope, &raw_input))
        .product();
    println!("{}", product)
}

fn count_trees((x, y): (usize, usize), input: &str) -> usize {
    let row_len = input.lines().next().unwrap_or("").len();
    let mut indexer = (0..row_len).cycle();
    indexer.next(); // first step is from 0, not to 0

    input
        .lines()
        .skip(y)
        .step_by(y)
        .filter(|line| {
            indexer
                .nth(x - 1)
                .and_then(|n| line.chars().nth(n))
                .map(|c| c == '#')
                .unwrap_or(false)
        })
        .count()
}
