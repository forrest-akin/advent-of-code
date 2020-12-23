use std::collections::HashMap;
use std::fs;
use std::iter;


#[derive(Debug)]
struct BagRule {
    bag_type: String,
    contents: HashMap<String, usize>,
}

impl BagRule {
    fn sum_descendants<'a>(
        &'a self,
        bag_map: &'a HashMap<String, &BagRule>,
        multiple: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        Box::new(
            self.contents.values()
            .map(move |count| count * multiple)
            .chain(self.contents.iter().flat_map(move |(bag_type, count)| {
                match bag_map.get(bag_type) {
                    Some(bag_rule) => bag_rule.sum_descendants(bag_map, count * multiple),
                    None => Box::new(iter::empty()),
                }
            })),
        )
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("IOError: unable to read input");
    let bag_rules: Vec<BagRule> = input.lines().map(parse_rule).collect();
    let sum_of_descendants = to_sum_of_descendants("shiny gold", &bag_rules);
    println!("{}", sum_of_descendants);
}

fn to_sum_of_descendants(bag_type: &str, bag_rules: &[BagRule]) -> usize {
    let bag_map = bags_by_type(bag_rules);
    match bag_map.get(bag_type) {
        Some(bag_rule) => bag_rule.sum_descendants(&bag_map, 1).sum(),
        None => 0,
    }
}

fn bags_by_type(bag_rules: &[BagRule]) -> HashMap<String, &BagRule> {
    bag_rules.iter().fold(HashMap::new(), |mut map, bag_rule| {
        map.insert(bag_rule.bag_type.to_string(), bag_rule);
        map
    })
}

fn parse_rule(line: &str) -> BagRule {
    line.split(" bags contain ")
    .collect::<Vec<&str>>()
    .split_first()
    .map(|(bag_type, raw_contents)| BagRule {
        bag_type: bag_type.to_string(),
        contents: parse_bag_contents(&raw_contents.join("")),
    })
    .expect("ParseError: unable to parse bag rule")
}

fn parse_bag_contents(raw_contents: &str) -> HashMap<String, usize> {
    raw_contents.split(", ")
    .fold(HashMap::new(), |mut map, raw_content| {
        raw_content.split(' ')
        .collect::<Vec<&str>>()
        .split_first()
        .and_then(|(raw_quantity, rest)| {
            rest.split_last()
            .map(|(_, type_parts)| type_parts.join(" "))
            .map(|bag_type| {
                raw_quantity.parse()
                .map(|quantity: usize| map.insert(bag_type, quantity))
            })
        });
        map
    })
}
