mod data;

use std::collections::HashMap;

fn main() {
    let data = data::get_data();
    //let data = data::_get_sample();

    //puzzle(data, 2020); // puzzle 1
    puzzle(data, 30000000); // puzzle 2
}

fn puzzle(data: Vec<usize>, stop: usize) {
    let mut numbers = vec![];
    let mut last_num = 0;
    let mut numbers_last_seen_turn: HashMap<usize, usize> = HashMap::new();

    for turn in 1..=stop {
        let num;

        if turn <= data.len() {
            num = data[turn - 1];
        } else {
            if numbers_last_seen_turn.contains_key(&last_num) {
                num = turn - *numbers_last_seen_turn.get(&last_num).unwrap() - 1;
            } else {
                num = 0;
            }
        }

        if turn == stop {
            println!("turn {}: {}", turn, num);
        }

        numbers_last_seen_turn.insert(last_num, turn - 1);
        last_num = num;

        numbers.push(num);
    }
}
