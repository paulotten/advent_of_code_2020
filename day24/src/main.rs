mod data;

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn parse(str: &str) -> Point {
        let mut x = 0;
        let mut y = 0;

        let mut n = false;
        let mut s = false;

        for c in str.chars() {
            match c {
                'n' => {
                    y += 1;

                    n = true;
                    s = false;
                },
                's' => {
                    y -= 1;

                    n = false;
                    s = true;
                },
                'w' => {
                    if !s {
                        x -= 1;
                    }

                    n = false;
                    s = false;
                },
                'e' => {
                    if !n {
                        x += 1;
                    }

                    n = false;
                    s = false;
                },
                _ => panic!("Unsupported direction: {}", c),
            };
        }

        Point{x, y}
    }
}

fn main() {
    let data = data::get_data();
    let data = data::_sample();

    let points: Vec<_> = data.lines().map(Point::parse).collect();

    puzzle1(&points);
}

fn setup(points: &Vec<Point>) -> HashSet<Point> {
    let mut set: HashSet<Point> = HashSet::new();

    for p in points {
        if set.contains(p) {
            set.remove(p);
        } else {
            set.insert(*p);
        }
    }

    set
}

fn puzzle1(points: &Vec<Point>) {
    println!("{}", setup(points).iter().count());
}