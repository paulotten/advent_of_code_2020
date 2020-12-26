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

    fn neighbors(&self, all_points: &HashSet<Point>) -> u8 {
        let mut count = 0;

        for x in -1..=1 {
            for y in -1..=1 {
                if x == y {
                    continue;
                }

                let test_point = Point{x: self.x + x, y: self.y + y};

                if all_points.contains(&test_point) {
                    count += 1;
                }
            }
        }

        count
    }
}

fn main() {
    let data = data::get_data();
    //let data = data::_sample();

    let points: Vec<_> = data.lines().map(Point::parse).collect();

    puzzle2(&points);
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

fn _puzzle1(points: &Vec<Point>) {
    println!("{}", setup(points).iter().count());
}

fn puzzle2(points: &Vec<Point>) {
    let mut points = setup(points);

    println!("Day 0: {}", points.iter().count());

    for i in 1..=100 {
        let bounds = get_bounds(&points);
        let min = bounds.0;
        let max = bounds.1;

        let mut new_points: HashSet<Point> = HashSet::new();

        for x in min.x-1 ..= max.x+1 {
            for y in min.y-1 ..= max.y+1 {
                let test_point = Point{x, y};
                let neighbors = test_point.neighbors(&points);

                if points.contains(&test_point) {
                    if neighbors == 1 || neighbors == 2 {
                        new_points.insert(test_point);
                    }
                } else {
                    if neighbors == 2 {
                        new_points.insert(test_point);
                    }
                }
            }
        }

        points = new_points;

        println!("Day {}: {}", i, points.iter().count());
    }
}

fn get_bounds(points: &HashSet<Point>) -> (Point, Point) {
    let mut min = Point{x: 0, y: 0};
    let mut max = Point{x: 0, y: 0};

    for p in points {
        if p.x > max.x {
            max.x = p.x;
        } else if p.x < min.x {
            min.x = p.x;
        }

        if p.y > max.y {
            max.y = p.y;
        } else if p.y < min.y {
            min.y = p.y;
        }
    }

    (min, max)
}
