mod data;

use std::mem::swap;

struct Direction {
    action: char,
    value: i32,
}

impl Direction {
    fn parse(input: &str) -> Direction {
        let action = input.chars().next().unwrap();
        let value = input[1..].parse().unwrap();

        Direction {
            action,
            value,
        }
    }
}

fn main() {
    let data = data::get_data();
    //let data = data::_sample_data();

    let directions = data.lines().map(Direction::parse).collect();

    puzzle2(directions);
}

fn puzzle2(directions: Vec<Direction>) {
    // waypoint
    let mut x: i32 = 10;
    let mut y: i32 = 1;
    // ship
    let mut ship_x: i32 = 0;
    let mut ship_y: i32 = 0;

    for direction in directions {
        match direction.action {
            'N' => y += direction.value,
            'S' => y -= direction.value,
            'E' => x += direction.value,
            'W' => x -= direction.value,
            'L' => {
                let coord = left(x, y, direction.value as u32);
                x = coord.0;
                y = coord.1;
            },
            'R' => {
                let coord = right(x, y, direction.value as u32);
                x = coord.0;
                y = coord.1;
            },
            'F' => {
                ship_x += x * direction.value;
                ship_y += y * direction.value;
            },
            a @ _ => panic!("Invalid action: {}", a),
        }
    }

    println!("{}, {} ({})", ship_x, ship_y, ship_x.abs() + ship_y.abs());
}

fn right(x: i32, y: i32, degrees: u32) -> (i32, i32) {
    rotate(x, y, degrees, true)
}

fn left(x: i32, y: i32, degrees: u32) -> (i32, i32) {
    rotate(x, y, degrees, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right() {
        let mut coords = (10, 4);

        coords = right(coords.0, coords.1, 90);
        assert_eq!(coords, (4, -10));
        coords = right(coords.0, coords.1, 90);
        assert_eq!(coords, (-10, -4));
        coords = right(coords.0, coords.1, 90);
        assert_eq!(coords, (-4, 10));
        coords = right(coords.0, coords.1, 90);
        assert_eq!(coords, (10, 4));
        
        coords = right(coords.0, coords.1, 180);
        assert_eq!(coords, (-10, -4));
    }
}

fn rotate(mut x: i32, mut y: i32, mut degrees: u32, right: bool) -> (i32, i32) {
    if degrees % 90 != 0 {
        panic!("Degrees must be a multiple of 90, was {}", degrees);
    }

    while degrees > 0 {
        if right {
            x = -x;
        } else {
            y = -y;
        }

        swap(&mut x, &mut y);

        degrees -= 90;
    }

    (x, y)
}

fn _puzzle1(directions: Vec<Direction>) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut current_direction = 'E';

    for direction in directions {
        match direction.action {
            'N' => y += direction.value,
            'S' => y -= direction.value,
            'E' => x += direction.value,
            'W' => x -= direction.value,
            'L' => current_direction = _turn(current_direction, -direction.value),
            'R' => current_direction = _turn(current_direction, direction.value),
            'F' => match current_direction {
                'N' => y += direction.value,
                'S' => y -= direction.value,
                'E' => x += direction.value,
                'W' => x -= direction.value,
                d @ _ => panic!("Invalid current direction: {}", d),
            },
            a @ _ => panic!("Invalid action: {}", a),
        }
    }

    println!("{}, {} ({})", x, y, x.abs() + y.abs());
}

fn _turn(starting_direction: char, mut degrees: i32) -> char {
    let directions = ['N', 'E', 'S', 'W'];
    let i = directions.iter().position(|x| *x == starting_direction);

    if i.is_none() {
        panic!("Invalid starting direction: {}", starting_direction);
    }

    if degrees % 90 != 0 {
        panic!("Degrees must be a multiple of 90, was {}", degrees);
    }

    let mut i = i.unwrap();

    if degrees < 0 {
        while degrees < 0 {
            if i == 0 {
                i = directions.len();
            }

            i -= 1;
            degrees += 90;
        }
    } else {
        while degrees > 0 {
            i = (i + 1) % directions.len();
            degrees -= 90;
        }
    }

    directions[i]
}
