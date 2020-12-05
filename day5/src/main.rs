mod data;

use std::collections::HashMap;

fn main() {
    let data = data::get_data();
    
    puzzle2(data);
}

fn _puzzle1(data: &str) {
    let mut max = 0;

    for line in data.lines() {
        let mut current = 0;

        for c in line.chars() {
            current *= 2;

            current += match c {
                'B' | 'R' => 1,
                _ => 0,
            };
        }

        if current > max {
            max = current;
        }
    }

    println!("{}", max);
}

fn puzzle2(data: &str) {
    let mut seats: HashMap<u32, ()> = HashMap::new();

    for line in data.lines() {
        let mut current = 0;

        for c in line.chars() {
            current *= 2;

            current += match c {
                'B' | 'R' => 1,
                _ => 0,
            };
        }

        seats.insert(current, ());
    }

    for i in 0..=828 {
        if !seats.contains_key(&i) {
            println!("{}", i);
        }
    }
}
