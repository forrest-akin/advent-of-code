use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;


fn main() {
    let input = fs::read_to_string("input").expect("IOError: unable to read input");
    let bag_rules = input.lines().map(parse_rule).collect();
    let sum_of_descendants = to_sum_of_descendants("shiny gold", bag_rules);
    println!("{}", sum_of_descendants);
}

fn to_sum_of_descendants(bag_type: &str, bag_rules: Vec<BagRule>) -> usize {
    sum_descendants( &bags_by_type(bag_rules), 0, &bag_type.to_string() )
}

fn bags_by_type(bag_rules: Vec<BagRule>) -> HashMap<String, HashMap<String, usize>> {
    bag_rules.iter().fold(HashMap::new(), |mut map, bag_rule| {
        map.insert( bag_rule.bag_type.to_string(), bag_rule.contents.clone() );
        map  
    })
}

fn sum_descendants( bag_map: &HashMap<String, HashMap<String, usize>>
                  , sum: usize
                  , bag_type: &String
                  ) -> usize {
    match bag_map.get(bag_type) {
        Some(contents) =>
            contents.iter().fold(sum, |x, (child_type, count)|
                x + count * sum_descendants(bag_map, 1, child_type)),
        _ => sum,
    }
}

fn parse_rule(line: &str) -> BagRule {
    line.split(" bags contain ").collect::<Vec<&str>>()
    .split_first()
    .map(|(bag_type, raw_contents)| BagRule {
        bag_type: bag_type.to_string(),
        contents: parse_bag_contents(&raw_contents.join(""))
    })
    .expect("ParseError: unable to parse bag rule")
}

fn parse_bag_contents(raw_contents: &str) -> HashMap<String, usize> {
    raw_contents.split(", ")
    .fold(HashMap::new(), |mut map, raw_content| {
        raw_content.split(" ").collect::<Vec<&str>>()
        .split_first()
        .and_then(|(raw_quantity, rest)|
            rest.split_last()
            .map(|(_, type_parts)| type_parts.join(" "))
            .map(|bag_type|
                raw_quantity.parse()
                .map(|quantity: usize| map.insert(bag_type, quantity))));
        map
    })
}

#[derive(Debug)]
struct BagRule {
    bag_type: String,
    contents: HashMap<String, usize>,
}
