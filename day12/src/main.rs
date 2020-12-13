mod data;

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

    puzzle1(directions);
}

fn puzzle1(directions: Vec<Direction>) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut current_direction = 'E';

    for direction in directions {
        match direction.action {
            'N' => y += direction.value,
            'S' => y -= direction.value,
            'E' => x += direction.value,
            'W' => x -= direction.value,
            'L' => current_direction = turn(current_direction, -direction.value),
            'R' => current_direction = turn(current_direction, direction.value),
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

fn turn(starting_direction: char, mut degrees: i32) -> char {
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
