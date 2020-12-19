use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;


fn main() {
    let input = fs::read_to_string("input").expect("IOError: unable to read input");
    let bag_rules = input.lines().map(parse_rule).collect();
    let shiny_gold_ancestors = to_ancestor_set("shiny gold", bag_rules);
    println!("{}", shiny_gold_ancestors.len());
}

fn to_ancestor_set(bag_type: &str, bag_rules: Vec<BagRule>) -> HashSet<String> {
    insert_ancestors( &bags_by_children(bag_rules)
                    , HashSet::new()
                    , &bag_type.to_string() )
}

fn bags_by_children(bag_rules: Vec<BagRule>) -> HashMap<String, HashSet<String>> {
    bag_rules.iter().fold(HashMap::new(), |map, parent| {
        parent.contents.keys().fold(map, |mut _map, child_bag_type| {
            if !_map.contains_key(child_bag_type)
                { _map.insert( child_bag_type.to_string(), HashSet::new() ); }
            if let Some(set) = _map.get_mut(child_bag_type)
                { set.insert( parent.bag_type.to_string() ); }
            _map
        })
    })
}

fn insert_ancestors( container_sets_map: &HashMap<String, HashSet<String>>
                   , set: HashSet<String>
                   , bag_type: &String
                   ) -> HashSet<String> {
    match container_sets_map.get(bag_type) {
        Some(container_set) =>
            container_set.iter().fold(set, |s, bag_type| {
                insert_bag(container_sets_map, s, &bag_type)
            }),
        _ => set,
    }
}

fn insert_bag( container_sets_map: &HashMap<String, HashSet<String>>
             , mut set: HashSet<String>
             , bag_type: &String
             ) -> HashSet<String> {
    if set.contains(bag_type) { return set; }
    else { set.insert( bag_type.to_string() ); }
    insert_ancestors(container_sets_map, set, bag_type)
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
