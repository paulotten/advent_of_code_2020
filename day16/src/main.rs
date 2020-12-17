mod data;

use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone)]
struct Range {
    pub min: u64,
    pub max: u64,
}

impl Range {
    fn parse(data: &str) -> Range {
        let parts: Vec<_> = data.split("-").collect();
        assert!(parts.len() == 2);

        Range {
            min: parts[0].parse().unwrap(),
            max: parts[1].parse().unwrap(),
        }
    }
}

fn main() {
    let data = data::get_data();
    //let data = data::_sample_data();

    let parts: Vec<_> = data.split("\n\n").collect();
    assert!(parts.len() == 3);

    let valid_fields = get_valid_fields(parts[0]);
    let my_ticket = parse_ticket(parts[1].lines().skip(1).next().unwrap());
    let nearby_tickets: Vec<_> = parts[2].lines().skip(1).map(parse_ticket).collect();

    //_puzzle1(&valid_fields, &nearby_tickets);

    puzzle2(&valid_fields, &nearby_tickets, &my_ticket);
}

fn puzzle2(
    valid_fields: &HashMap<&str, Vec<Range>>,
    nearby_tickets: &Vec<Vec<u64>>,
    my_ticket: &Vec<u64>,
) {
    // discard invalid tickets
    let valid_ranges: Vec<_> = valid_fields.values().flatten().map(|x| *x).collect();

    let valid_tickets: Vec<_> = nearby_tickets
        .iter()
        .filter(|x| {
            for value in x.iter() {
                if !is_valid(&valid_ranges, *value) {
                    return false;
                }
            }

            true
        })
        .collect();

    // figure out possible field locations
    let mut possiblities: HashMap<&str, HashSet<usize>> = HashMap::new();
    
    for (name, ranges) in valid_fields {
        let mut set = HashSet::new();

        // all tickets are the same length
        // therefor can use the length of mine
        for i in 0..my_ticket.len() {
            for j in 0..valid_tickets.len() {
                if !is_valid(ranges, valid_tickets[j][i]) {
                    break;
                }

                // all the tickets match this field
                if j == valid_tickets.len() - 1 {
                    set.insert(i);
                }
            }
        }

        possiblities.insert(name, set);
    }

    // use a process of elimination to figure out definate field locations
    let mut locations: HashMap<&str, usize> = HashMap::new();

    loop {
        if possiblities.len() == 0 {
            break;
        }

        let mut found_location = false;
        let mut key_to_remove = "";
        let mut index_to_remove = 0;

        for (key, values) in &possiblities {
            if values.len() == 1 {
                found_location = true;
                key_to_remove = key;
                index_to_remove = *values.iter().next().unwrap();
            }
        }

        assert!(found_location);

        // add found location to list
        locations.insert(key_to_remove, index_to_remove);

        // remove it from possibilities
        possiblities.remove(key_to_remove);

        // need to dereference keys before we start mutating the HashMap
        let keys: Vec<_> = possiblities.keys().map(|x| *x).collect();

        for key in keys {
            possiblities.get_mut(key).unwrap().remove(&index_to_remove);
        }
    }

    let mut mult = 1;

    for (name, _) in valid_fields {
        if name.starts_with("departure") {
            mult *= my_ticket[*locations.get(name).unwrap()];
        }
    }

    println!("{}", mult);
}

fn _puzzle1(valid_fields: &HashMap<&str, Vec<Range>>, nearby_tickets: &Vec<Vec<u64>>) {
    let valid_ranges: Vec<_> = valid_fields.values().flatten().map(|x| *x).collect();

    let sum: u64 = nearby_tickets
        .iter()
        .flatten()
        .filter(|x| !is_valid(&valid_ranges, **x))
        .sum();

    println!("{}", sum);
}

fn is_valid(valid_ranges: &Vec<Range>, value: u64) -> bool {
    for range in valid_ranges {
        if value >= range.min && value <= range.max {
            return true;
        }
    }

    false
}

fn get_valid_fields(data: &str) -> HashMap<&str, Vec<Range>> {
    let mut valid_fields: HashMap<&str, Vec<Range>> = HashMap::new();

    for line in data.lines() {
        let parts: Vec<_> = line.split(": ").collect();
        assert!(parts.len() == 2);

        let name = parts[0];
        let ranges = parts[1].split(" or ").map(Range::parse).collect();

        valid_fields.insert(name, ranges);
    }

    valid_fields
}

fn parse_ticket(data: &str) -> Vec<u64> {
    data.split(",").map(|x| x.parse().unwrap()).collect()
}
