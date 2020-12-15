mod data;

use std::collections::HashMap;

fn main() {
    let data = data::get_data();
    //let data = data::_sample_data2();

    let commands: Vec<Vec<_>> = data.lines().map(|x| x.split(" = ").collect()).collect();

    puzzle2(commands);
}

fn puzzle2(commands: Vec<Vec<&str>>) {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask: Vec<Option<u64>> = vec![];

    for command in commands {
        match command[0] {
            "mask" => {
                mask = get_mask(command[1]);
            }
            s if s.starts_with("mem[") => {
                let mut base_address = get_address(s);
                let value: u64 = command[1].parse().unwrap();

                // apply mask

                // pass 1
                // figure out our base address
                let mut mult = 1;

                for m in &mask {
                    if m.is_some() {
                        let m = m.unwrap();
                        
                        // overwrite for 1
                        if m == 1 {
                            if base_address / mult % 2 != 1 {
                                base_address += mult;
                            }
                        }
                        // do nothing for 0
                    } else {
                        // zero for X
                        if base_address / mult % 2 != 0 {
                            base_address -= mult;
                        }
                    }

                    mult *= 2;
                }

                // pass 2
                // figure out all address by multiplying in mask
                let mut addresses = vec!(base_address);
                let mut mult = 1;

                for m in &mask {
                    if m.is_none() {
                        let mut new_addresses = vec!();

                        for a in &addresses {
                            new_addresses.push(a + mult);
                        }

                        addresses.append(&mut new_addresses);
                    }

                    mult *= 2;
                }

                for address in addresses {
                    memory.insert(address, value);
                }
            }
            x @ _ => panic!("Unsupported command: {}", x),
        }
    }

    println!("{}", memory.values().sum::<u64>());
}

fn get_mask(s: &str) -> Vec<Option<u64>> {
    let mut mask = vec![];

    for c in s.chars().rev() {
        mask.push(match c {
            '0' => Some(0),
            '1' => Some(1),
            'X' => None,
            _ => panic!("Unsupported mask character: {}", c),
        })
    }

    mask
}

fn get_address(s: &str) -> u64 {
    s.split("mem[")
        .skip(1)
        .next()
        .unwrap()
        .split(']')
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

fn _puzzle1(commands: Vec<Vec<&str>>) {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask: Vec<Option<u64>> = vec![];

    for command in commands {
        match command[0] {
            "mask" => {
                mask = get_mask(command[1]);
            }
            s if s.starts_with("mem[") => {
                let address = get_address(s);
                let mut value: u64 = command[1].parse().unwrap();

                // apply mask
                let mut mult = 1;

                for m in &mask {
                    if m.is_some() {
                        let m = m.unwrap();

                        if m == 0 {
                            if value / mult % 2 != 0 {
                                value -= mult;
                            }
                        } else if m == 1 {
                            if value / mult % 2 != 1 {
                                value += mult;
                            }
                        }
                    }

                    mult *= 2;
                }

                memory.insert(address, value);
            }
            x @ _ => panic!("Unsupported command: {}", x),
        }
    }

    println!("{}", memory.values().sum::<u64>());
}
