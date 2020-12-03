mod data;

fn main() {
    let data = data::get_data();

    puzzle2(data);
}

fn _puzzle1(data: &str) {
    println!("hit {} trees", sled(data, 3, 1));
}

fn puzzle2(data: &str) {
    let a = sled(data, 1, 1);
    let b = sled(data, 3, 1);
    let c = sled(data, 5, 1);
    let d = sled(data, 7, 1);
    let e = sled(data, 1, 2);

    println!(
        "{} * {} * {} * {} * {} = {}",
        a,
        b,
        c,
        d,
        e,
        a * b * c * d * e
    );
}

fn sled(data: &str, right: usize, down: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;

    for line in data.lines() {
        if y % down != 0 {
            y += 1;
            continue;
        }

        let chars: Vec<char> = line.chars().collect();

        while x >= chars.len() {
            x -= chars.len();
        }

        if chars[x] == '#' {
            trees += 1;
        }

        x += right;
        y += 1;
    }

    trees
}
