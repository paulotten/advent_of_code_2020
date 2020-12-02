mod data;

use data::Entry;

fn main() {
    let entries = data::get_entries();

    part2(&entries);
}

fn _part1(entries: &Vec<Entry>) {
    let mut count = 0;

    for entry in entries {
        let occurances = entry.password.matches(entry.letter).count();

        if occurances >= entry.min && occurances <= entry.max {
            count += 1;
        }
    }

    println!("{} valid passwords", {count});
}

fn part2(entries: &Vec<Entry>) {
    let mut count = 0;

    for entry in entries {
        let mut occurances = 0;

        // min and max don't make sense any more since part 2 redefines the data structure
        if entry.password.chars().nth(entry.min-1).unwrap() == entry.letter {
            occurances += 1;
        }
        if entry.password.chars().nth(entry.max-1).unwrap() == entry.letter {
            occurances += 1;
        }

        if occurances == 1 {
            count += 1;
        }
    }

    println!("{} valid passwords", {count});
}
