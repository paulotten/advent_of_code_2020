mod data;

#[derive(Debug)]
struct Tile {
    id: u32,
    sides: Vec<String>,
}

impl Tile {
    fn parse(s: &str) -> Tile {
        let lines: Vec<_> = s.lines().collect();

        let parts: Vec<_> = lines[0].split("Tile ").collect();
        let parts: Vec<_> = parts[1].split(":").collect();
        let id = parts[0].parse().unwrap();

        let mut sides = vec![String::from(lines[1]), String::from(lines[lines.len() - 1])];

        let mut left = String::new();
        let mut right = String::new();

        for i in 1..lines.len() {
            let mut chars = lines[i].chars();

            left.push(chars.next().unwrap());
            right.push(chars.last().unwrap());
        }

        sides.push(left);
        sides.push(right);

        Tile {
            id,
            sides,
        }
    }

    fn matches(&self, other: &Tile) -> bool {
        for a in &self.sides {
            for b in &other.sides {
                if a == b || *a == b.chars().rev().collect::<String>() {
                    return true;
                }
            }
        }

        false
    }
}

fn main() {
    let data = data::get_data();
    //let data = data::_sample();

    let tiles: Vec<_> = data.split("\n\n").map(Tile::parse).collect();

    puzzle1(&tiles);
}

fn puzzle1(tiles: &Vec<Tile>) {
    let mut corners: Vec<u64> = vec![];

    for tile in tiles {
        let count_matches = get_count_matches(tile, tiles);

        // I only need the corners
        // the corners match exactly two other pieces
        if count_matches == 2 {
            corners.push(tile.id as u64);
        }
    }

    println!("{:?}", corners);
    println!("{}", corners.iter().product::<u64>());
}

fn get_count_matches(tile: &Tile, tiles: &Vec<Tile>) -> u8 {
    let mut count = 0;

    println!("{}", tile.id);

    for test in tiles {
        // don't match self
        if tile.id == test.id {
            continue;
        }

        if tile.matches(test) {
            println!("matches {}", test.id);
            count += 1;
        }
    }

    count
}
