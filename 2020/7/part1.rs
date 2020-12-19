use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;


fn main() {
    let input = fs::read_to_string("input").expect("IOError: unable to read input");
    let shiny_gold_container_set = to_set_of_containers(
        input.lines()
        .map(parse_rule)
        .collect());
    println!("{}", shiny_gold_container_set.len() - 1);
}

fn to_set_of_containers(bag_rules: Vec<BagRule>) -> HashSet<String> {
    let mut set = HashSet::new();
    hydrate_set(&"shiny gold".to_string(), &build_container_map(bag_rules), &mut set);
    set
}

fn build_container_map(bag_rules: Vec<BagRule>) -> HashMap<String, HashSet<String>> {
    bag_rules.iter()
    .fold(HashMap::new(), |mut map, parent| {
        parent.contents.keys()
        .for_each(|child_bag_type| {
            if !map.contains_key(child_bag_type) {
                map.insert(child_bag_type.to_string(), HashSet::new());
            }

            map.get_mut(child_bag_type)
            .map(|set| set.insert(parent.bag_type.to_string()));
        });
        map
    })
}

fn hydrate_set( key: &String
              , container_maps_map: &HashMap<String, HashSet<String>>
              , set: &mut HashSet<String>
              ) {
    if set.contains(key) { return; }
    else { set.insert(key.to_string()); }

    container_maps_map.get(key)
    .map(|container_map|
        container_map.iter()
        .for_each(|bag_type| {
            hydrate_set(&bag_type, container_maps_map, set);
        }));
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
