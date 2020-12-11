mod data;

use std::collections::{BTreeMap, HashMap};

fn main() {
    let data = data::get_data();
    //let data = data::_get_sample1();

    let mut data: Vec<u32> = data.lines().map(|l| l.parse().unwrap()).collect();
    data.sort();

    puzzle2_attempt2(data);
}

fn puzzle2_attempt2(mut data: Vec<u32>) {
    // lets start by figuring out what connects directly to what
    let mut direct_paths: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    // include the starting position
    data.insert(0, 0);

    // lets look at this backwards
    // we have to reach the last element, so lets start there
    for i in (0..data.len()).rev() {
        let mut paths = vec![];

        for j in 1..=3 {
            if (i as isize - j as isize) < 0 {
                break;
            }

            let k = i - j;

            if data[i] - data[k] <= 3 {
                paths.push(k);
            }
        }

        direct_paths.insert(i, paths);
    }

    // now lets figure out the number of (possibly indirect) paths to each node
    let mut num_paths: BTreeMap<usize, u64> = BTreeMap::new();

    for (i, paths) in direct_paths {
        let mut num = 0;

        for node in paths {
            if node == 0 {
                num += 1;
            } else {
                num += num_paths.get(&node).unwrap();
            }
        }

        num_paths.insert(i, num);
    }

    // now we just need to print the number of paths to the last node
    println!("{:?}", num_paths.get(&(num_paths.len() - 1)).unwrap());
}

// 50ms to get 20,000 combinations
// some napkin math suggests at a linear rate it could take 30 days to get a trillion
// and the answer is apparently at least a trillion...
fn _puzzle2(data: Vec<u32>) {
    println!("{}", _recurse(&data, 0, 0));
}

fn _recurse(data: &Vec<u32>, i: usize, joltage: u32) -> u32 {
    let mut valid_combinations = 0;

    for j in i..i + 3 {
        if j >= data.len() {
            // invalid combination
            // out of array bounds
        } else if data[j] - joltage > 3 {
            // invalid combination
            // too large a step
        } else if j == data.len() - 1 {
            valid_combinations += 1;
        } else {
            valid_combinations += _recurse(data, j + 1, data[j]);
        }
    }

    valid_combinations
}

fn _puzzle1(data: Vec<u32>) {
    let mut joltage = 0;
    let mut jumps: HashMap<u32, u32> = HashMap::new();

    for i in data {
        let jump = i - joltage;

        *jumps.entry(jump).or_insert(0) += 1;

        joltage = i;
    }

    // device can jump +3 jolts
    let jump = 3;
    *jumps.entry(jump).or_insert(0) += 1;
    joltage += jump;

    println!(
        "{} * {} = {} ({} jolts)",
        jumps.get(&1).unwrap(),
        jumps.get(&3).unwrap(),
        jumps.get(&1).unwrap() * jumps.get(&3).unwrap(),
        joltage
    );
    // 64 * 29 = 1856 (151 jolts)
}
