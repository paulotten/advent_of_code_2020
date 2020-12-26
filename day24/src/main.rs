mod data;

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn parse(s: &str) -> Point {
        let mut x = 0;
        let mut y = 0;

        let mut diag = false;

        for c in s.chars() {
            match c {
                'n' => {
                    y += 1;
                    diag = true;
                },
                's' => {
                    y -= 1;
                    diag = true;
                },
                'w' => {
                    if diag {
                        x -= 1;
                    } else {
                        x -= 2;
                    }

                    diag = false;
                },
                'e' => {
                    if diag {
                        x += 1;
                    } else {
                        x += 2;
                    }

                    diag = false;
                },
                _ => panic!("Unsupported direction: {}", c),
            };
        }

        Point{x, y}
    }
}

fn main() {
    let data = data::get_data();
    //let data = data::_sample();

    let points: Vec<_> = data.lines().map(Point::parse).collect();

    puzzle1(&points);
}

fn puzzle1(points: &Vec<Point>) {
    let mut set: HashSet<Point> = HashSet::new();

    for p in points {
        if set.contains(p) {
            set.remove(p);
        } else {
            set.insert(*p);
        }
    }

    println!("{}", set.iter().count());
}