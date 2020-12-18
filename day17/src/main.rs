mod data;

use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

struct Bounds {
    min: Point,
    max: Point,
}

fn main() {
    let data = data::get_data();
    //let data = data::_sample_data();

    let points = get_points(data);

    puzzle1(points);
}

fn puzzle1(mut points: HashSet<Point>) {
    let mut cycle = 0;

    loop {
        let bounds = get_bounds(&points);

        /*
        println!("Cycle {}", cycle);
        println!();
        _print_points(&points, &bounds);
        */

        cycle += 1;

        if cycle > 6 {
            break;
        }

        let mut new_points: HashSet<Point> = HashSet::new();

        for x in bounds.min.x-1 ..= bounds.max.x+1 {
            for y in bounds.min.y-1 ..= bounds.max.y+1 {
                for z in bounds.min.z-1 ..= bounds.max.z+1 {
                    let point = Point{x, y, z};
                    let neighbors = get_neighbors(&point, &points);

                    if points.get(&point).is_none() {
                        if neighbors == 3 {
                            new_points.insert(point);
                        }
                    } else {
                        if neighbors == 2 || neighbors == 3 {
                            new_points.insert(point);
                        }
                    }
                }
            }
        }

        points = new_points;
    }

    println!("{}", points.len());
}

fn get_neighbors(point: &Point, all_points: &HashSet<Point>) -> u8 {
    let mut count = 0;

    for x in point.x-1 ..= point.x+1 {
        for y in point.y-1 ..= point.y+1 {
            for z in point.z-1 ..= point.z+1 {
                let test_point = Point{x, y, z};

                if test_point == *point {
                    continue;
                }

                if all_points.contains(&test_point) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn _print_points(points: &HashSet<Point>, bounds: &Bounds) {
    for z in bounds.min.z..=bounds.max.z {
        println!("z={}", z);

        for y in bounds.min.y..=bounds.max.y {
            for x in bounds.min.x..=bounds.max.x {
                let c = match points.get(&Point{x, y, z}) {
                    Some(_) => '#',
                    None => '.',
                };
                print!("{}", c);
            }
            println!();
        }
        println!();
    }
}

fn get_bounds(points: &HashSet<Point>) -> Bounds {
    let mut min = Point{x: 0, y: 0, z: 0};
    let mut max = Point{x: 0, y: 0, z: 0};

    let mut first = true;

    for point in points {
        if first {
            min.x = point.x;
            min.y = point.y;
            min.z = point.z;

            max.x = point.x;
            max.y = point.y;
            max.z = point.z;

            first = false;
        } else {
            if point.x < min.x {
                min.x = point.x;
            } else if point.x > max.x {
                max.x = point.x;
            }
            
            if point.y < min.y {
                min.y = point.y;
            } else if point.y > max.y {
                max.y = point.y;
            }
            
            if point.z < min.z {
                min.z = point.z;
            } else if point.z > max.z {
                max.z = point.z;
            }
        }
    }

    Bounds{min, max}
}

fn get_points(data: &str) -> HashSet<Point> {
    let mut points: HashSet<Point> = HashSet::new();

    let z = 0;
    let lines: Vec<_> = data.lines().collect();

    for y in 0..lines.len() {
        let chars: Vec<_> = lines[y].chars().collect();

        for x in 0..chars.len() {
            if chars[x] == '#' {
                points.insert(Point{
                    x: x as i32, y: y as i32, z
                });
            }
        }
    }

    points
}
