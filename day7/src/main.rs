mod data;

use std::collections::{HashMap, HashSet};

struct Content {
    num: usize,
    color: String,
}

fn main() {
    //let data = data::_get_test_data();
    let data = data::get_data();
    let bag_contents = get_bag_contents(data);

    _puzzle2(bag_contents);
}

fn _puzzle2(bag_contents: HashMap<&str, Vec<Content>>) {
    // off by one again since we're counting the "shiny gold" bag
    println!("{}", sum_contents(&bag_contents, "shiny gold")-1);
}

fn sum_contents(bag_contents: &HashMap<&str, Vec<Content>>, bag: &str) -> usize {
    let mut sum = 1;

    for contents in bag_contents.get_key_value(bag).unwrap().1.iter() {
        sum += contents.num * sum_contents(&bag_contents, &contents.color);
    }

    sum
}

fn _puzzle1(bag_contents: HashMap<&str, Vec<Content>>) {
    // running list of all of the types of bags that can hold other bags
    let mut valid_bags = HashSet::<String>::new();
    // starting with a "shiny gold" bag
    // which we'll need to subtract for later
    valid_bags.insert(String::from("shiny gold"));

    // keep looping until we don't find another valid bag
    let mut old_num_bags = 0;
    let mut current_num_bags = valid_bags.len();

    while old_num_bags != current_num_bags {
        old_num_bags = current_num_bags;

        for (&bag, contents) in &bag_contents {
            if valid_bags.contains(bag) {
                continue;
            }

            let mut insert = false;

            'vb: for vb in &valid_bags {
                for content in contents {
                    if *vb == content.color {
                        insert = true;
                        break 'vb;
                    }
                }
            }

            if insert {
                valid_bags.insert(String::from(bag));
            }
        }

        current_num_bags = valid_bags.len();
    }

    println!("{}", valid_bags.len()-1);
}

fn get_bag_contents(data: &str) -> HashMap<&str, Vec<Content>> {
    let mut bags: HashMap<&str, Vec<Content>> = HashMap::new();
    
    for line in data.lines() {
        let parts: Vec<_> = line.split(" bags contain ").collect();
        assert!(parts.len() == 2);

        let color = parts[0];
        let mut contents = vec!();

        let mut parts = parts[1].split(' ').peekable();

        // the format appears to be "number, 2-word color, bag(s)"
        // so it looks like I can just read 4 words at a time
        while parts.peek().is_some() {
            let num = parts.next().unwrap();

            // "no other bags"
            if num == "no" {
                break;
            }
            
            let num: usize = num.parse().unwrap();

            let mut color = String::from(parts.next().unwrap());
            color.push(' ');
            color.push_str(parts.next().unwrap());

            // bag(s)
            assert!(parts.next().is_some());

            contents.push(Content{
                num,
                color,
            })
        }

        bags.insert(color, contents);
    }

    bags
}
