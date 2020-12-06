mod data;

use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let data = data::get_data();
    //let data = data::_sample_data();

    let groups = get_group_data(data);

    puzzle2(groups);
}

fn puzzle2(groups: Vec<Vec<&str>>) {
    let mut sum = 0;

    for group in groups {
        let group_size = group.len();
        let mut questions: HashMap<char, usize> = HashMap::new();

        for line in group {
            for c in line.chars() {
                *questions.entry(c).or_insert(0) += 1;
            }
        }

        for (_, value) in questions {
            if value == group_size {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}

fn _puzzle1(groups: Vec<Vec<&str>>) {
    let mut sum = 0;

    for group in groups {
        let mut questions: HashSet<char> = HashSet::new();

        for line in group {
            for c in line.chars() {
                questions.insert(c);
            }
        }

        sum += questions.len();
    }

    println!("{}", sum);
}

fn get_group_data(data: &str) -> Vec<Vec<&str>> {
    let mut groups: Vec<Vec<&str>> = vec!();
    let mut current_group: Vec<&str> = vec!();

    for line in data.lines() {
        if line.len() == 0 {
            groups.push(current_group);

            current_group = vec!();
            continue;
        }

        current_group.push(line);
    }
    groups.push(current_group);

    groups
}
