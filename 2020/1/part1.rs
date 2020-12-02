use std::collections::HashMap;
use std::fs;


fn main() {
    let input = fs::read_to_string("./input").expect("unable to read input!");
    let parsed: Vec<i32> = input.split("\n").map(|x| x.parse().unwrap()).collect();
    let hashed: HashMap<i32, i32> = parsed.iter().map(|&x| (2020 - x, x)).collect();
    let y = parsed.iter().find_map(|x| hashed.get(&x)).unwrap();
    let x = hashed.get(y).unwrap();

    println!("entries=({}, {})", x, y);
    println!("answer={}", x * y);
}
