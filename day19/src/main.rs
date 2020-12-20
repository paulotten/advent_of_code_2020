mod data;

use std::collections::HashMap;
use regex::Regex;

fn main() {
    let data = data::get_data();
    //let data = data::_sample3();

    let parts: Vec<_> = data.split("\n\n").collect();
    assert!(parts.len() == 2);

    let rules = parts[0];
    let messages = parts[1];

    let rules = map_rules(rules);

    puzzle2(&rules, messages);
}

fn map_rules(rules: &str) -> HashMap<u32, &str> {
    let mut rules_map: HashMap<u32, &str> = HashMap::new();

    for rule in rules.lines() {
        let parts: Vec<_> = rule.split(": ").collect();
        assert!(parts.len() == 2);

        rules_map.insert(parts[0].parse().unwrap(), parts[1]);
    }

    rules_map
}

fn _puzzle1(rules: &HashMap<u32, &str>, messages: &str) {
    let regex = format!("^{}$", resolve_rule(0, &rules));
    println!("{}", regex);

    let regex = Regex::new(&regex).unwrap();

    let count = messages.lines().filter(|l| regex.is_match(l)).count();
    println!("{}", count);
}

fn puzzle2(rules: &HashMap<u32, &str>, messages: &str) {
    let mut rules = rules.clone();

    let rule31 = resolve_rule(31, &rules);
    let rule42 = resolve_rule(42, &rules);

    // 8: 42 | 42 8
    let mut rule8 = String::from("\"(");
    rule8.push_str(&rule42);
    rule8.push_str(")+\"");

    rules.insert(8, &rule8);

    // 11: 42 31 | 42 11 31
    let mut rule11 = String::from("\"");
    rule11.push_str(&rule42);

    // how do you do internal repetition with regex?
    // no idea, lets just do an arbitary number of levels
    let levels = 10;
    
    for _ in 0..levels {
        rule11.push_str("(");
        rule11.push_str(&rule42);
    }
    for _ in 0..levels {
        rule11.push_str(&rule31);
        rule11.push_str(")?");
    }

    rule11.push_str(&rule31);
    rule11.push_str("\"");

    rules.insert(11, &rule11);

    let regex = format!("^{}$", resolve_rule(0, &rules));
    let regex = Regex::new(&regex).unwrap();

    let count = messages.lines().filter(|l| regex.is_match(l)).count();
    println!("{}", count);
}

fn resolve_rule(rule_number: u32, all_rules: &HashMap<u32, &str>) -> String {
    let rule = all_rules.get(&rule_number).unwrap();
    let mut resolved = String::new();

    let maybe_literal = rule.find('"');

    if maybe_literal.is_some() {
        let i = maybe_literal.unwrap() + 1;

        resolved.push_str(&rule[i..rule.rfind('"').unwrap()]);
    } else {
        let maybe_or = rule.find('|');

        if maybe_or.is_some() {
            resolved.push('(');

            let mut first = true;

            for part in rule.split(" | ") {
                if first {
                    first = false;
                } else {
                    resolved.push('|');
                }

                for rule_number in part.split(" ") {
                    resolved.push_str(&resolve_rule(rule_number.parse().unwrap(), all_rules));
                }
            }

            resolved.push(')');
        } else {
            for rule_number in rule.split(" ") {
                resolved.push_str(&resolve_rule(rule_number.parse().unwrap(), all_rules));
            }
        }
    }

    resolved
}
