use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("IOError: unable to read input");
    let groups = parse_input(&input);
    let uniq_answers_per_group = collect_common_answers(&groups);
    let sum: usize = uniq_answers_per_group.iter().map(|group| group.len()).sum();
    println!("{:?}", sum);
}

fn collect_common_answers(groups: &Vec<Vec<HashSet<char>>>) -> Vec<HashSet<char>> {
    groups.iter().map(|group|
        group.iter()
        .fold(group.first().unwrap().clone(), |x, y| x.intersection(y).cloned().collect()))
    .collect()
}

fn parse_input(input: &str) -> Vec<Vec<HashSet<char>>> {
    input.split("\n\n")
    .map(|group|
        group.lines()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .collect())
    .collect()
}
