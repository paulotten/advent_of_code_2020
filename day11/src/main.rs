mod data;

type SeatingMap = Vec<Vec<Option<bool>>>;

fn main() {
    let data = data::get_data();
    //let data = data::_sample_data();

    let seating = get_initial_seating(data);
    puzzle2(seating);
}

fn puzzle2(mut seating: SeatingMap) {
    let mut changed = true;

    while changed {
        changed = false;
        let mut new_seating = seating.clone();

        for i in 1..seating.len()-1 {
            for j in 1..seating[i].len()-1 {
                if seating[i][j] == None {
                    continue;
                }

                // so now with the new line of sight checking requirement my bounds checking avoidance is pretty useless
                let surrounding = vec!(
                    see_chair(&seating, i, j, -1, -1), see_chair(&seating, i, j, -1, 0), see_chair(&seating, i, j, -1, 1),
                    see_chair(&seating, i, j,  0, -1), /*            self             */ see_chair(&seating, i, j,  0, 1),
                    see_chair(&seating, i, j,  1, -1), see_chair(&seating, i, j,  1, 0), see_chair(&seating, i, j,  1, 1),
                );
                let count = surrounding.iter().filter(|x| **x).count();

                if seating[i][j] == Some(false) {
                    if count == 0 {
                        changed = true;
                        new_seating[i][j] = Some(true);
                    }
                } else if seating[i][j] == Some(true) {
                    if count >= 5 {
                        changed = true;
                        new_seating[i][j] = Some(false);
                    }
                }
            }
        }

        seating = new_seating;
    }

    println!("{}", seating.iter().flatten().filter(|x| **x == Some(true)).count());
}

fn see_chair(seating: &SeatingMap, x: usize, y: usize, x_vel: isize, y_vel: isize) -> bool {
    let x = (x as isize + x_vel) as usize;
    let y = (y as isize + y_vel) as usize;

    // might as well make tiny use out of my empty floor border
    if y == 0 || x == 0 || x == seating.len() - 1 || y == seating[x].len() - 1 {
        return false;
    }

    if seating[x][y].is_some() {
        return seating[x][y].unwrap();
    }

    return see_chair(seating, x, y, x_vel, y_vel);
}

fn _puzzle1(mut seating: SeatingMap) {
    let mut changed = true;

    while changed {
        changed = false;
        let mut new_seating = seating.clone();

        for i in 1..seating.len()-1 {
            for j in 1..seating[i].len()-1 {
                if seating[i][j] == None {
                    continue;
                }

                let surrounding = vec!(
                    seating[i-1][j-1], seating[i-1][j+0], seating[i-1][j+1],
                    seating[i+0][j-1], /*     self     */ seating[i+0][j+1],
                    seating[i+1][j-1], seating[i+1][j+0], seating[i+1][j+1],
                );
                let count = surrounding.iter().filter(|x| **x == Some(true)).count();

                if seating[i][j] == Some(false) {
                    if count == 0 {
                        changed = true;
                        new_seating[i][j] = Some(true);
                    }
                } else if seating[i][j] == Some(true) {
                    if count >= 4 {
                        changed = true;
                        new_seating[i][j] = Some(false);
                    }
                }
            }
        }

        seating = new_seating;
    }

    println!("{}", seating.iter().flatten().filter(|x| **x == Some(true)).count());
}

fn _print_seating(seating: & SeatingMap) {
    for i in 0..seating.len() {
        for j in 0..seating[i].len() {
            print!("{}", match seating[i][j] {
                Some(true) => '#',
                Some(false) => 'L',
                _ => '.',
            })
        }

        println!();
    }
}

fn get_initial_seating(data: &'static str) -> SeatingMap {
    let mut seating: SeatingMap = vec!();
    
    let mut y = 0;
    let mut line_len = 0;

    for line in data.lines() {
        // lets add a single layer floor around the outside so we don't have to bounds check later
        if y == 0 {
            line_len = line.len() + 2;

            seating.push(vec!(None; line_len));
            y += 1;
        }

        let mut row: Vec<Option<bool>> = vec!(None);

        for c in line.chars() {
            row.push(match c {
                'L' => Some(false),
                '#' => Some(true),
                _ => None,
            });
        }

        row.push(None);

        seating.push(row);
        y += 1;
    }

    // bottom layer of floor
    seating.push(vec!(None; line_len));

    seating
}
