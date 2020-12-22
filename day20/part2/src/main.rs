mod data;

fn main() {
    let monsters = data::get_monsters();

    let data = data::get_data();
    //let data = data::_sample();

    let puzzle = boolify(&remove_borders(data));

    let mut matches = 0;

    for monster in &monsters {
        let monster = boolify(monster);

        for i in 0..=(puzzle.len() - monster.len()) {
            for j in 0..=(puzzle[0].len() - monster[0].len()) {
                if is_monster(&puzzle, &monster, i, j) {
                    matches += 1;
                }
            }
        }
    }

    let waves = puzzle.iter().flatten().filter(|x| **x).count();
    // doesn't account for the possiblity of overlapping monsters
    let monsters = boolify(monsters[0]).iter().flatten().filter(|x| **x).count()*matches;

    println!("{}", waves - monsters);
}

fn is_monster(puzzle: &Vec<Vec<bool>>, monster: &Vec<Vec<bool>>, i: usize, j: usize) -> bool {
    for i2 in 0..monster.len() {
        for j2 in 0..monster[0].len() {
            if monster[i2][j2] && !puzzle[i+i2][j+j2] {
                return false;
            }
        }
    }

    true
}

fn boolify(input: &str) -> Vec<Vec<bool>> {
    let mut output = vec![];

    for line in input.lines() {
        let mut row = vec![];

        for c in line.chars() {
            row.push(match c {
                '#' => true,
                _ => false,
            });
        }

        output.push(row);
    }

    output
}

fn remove_borders(input: &str) -> String {
    let mut output = String::new();

    let mut i = 0;
    let mut j = 0;

    for line in input.lines() {
        if !is_border(i) {
            for c in line.chars() {
                if !is_border(j) {
                    output.push(c);
                }

                j += 1;
            }

            output.push('\n');
        }

        i += 1;
    }

    output
}

fn is_border(index: usize) -> bool {
    match index % 10 {
        0 | 9 => true,
        _ => false,
    }
}
