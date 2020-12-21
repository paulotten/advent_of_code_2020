use std::io;

const TILE: &str =
".#..##.#.#
#........#
#.#.#....#
.##.#..#..
##....##..
##.....#..
.#.#..#..#
.#......##
#.#......#
.#######..
";

// it's a 144 peice puzzle and I know which piece fits which
// how long could it take to just manually assemble it?
fn main() {
    let mut s = String::from(TILE);

    loop {
        println!("{}", s);
        println!("1 - up / down");
        println!("2 - left / right");
        println!("3 - x / y");

        let mut input = String::new();
        let _read = io::stdin().read_line(&mut input);

        match input.chars().next().unwrap() {
            // up / down
            '1' => {
                let mut new_string = String::new();

                for line in s.lines().rev() {
                    new_string.push_str(line);
                    new_string.push('\n');
                }

                s = new_string;
            },
            // left / right
            '2' => {
                let mut new_string = String::new();

                for line in s.lines() {
                    for c in line.chars().rev() {
                        new_string.push(c);
                    }
                    new_string.push('\n');
                }

                s = new_string;
            },
            '3' => {
                let mut chars: Vec<Vec<char>> = vec![];

                for line in s.lines() {
                    let mut line_chars: Vec<char> = vec![];

                    for c in line.chars() {
                        line_chars.push(c);
                    }

                    chars.push(line_chars);
                }

                let mut new_string = String::new();

                for a in 0..chars.len() {
                    for b in 0..chars[a].len() {
                        new_string.push(chars[b][a]);
                    }
                    new_string.push('\n');
                }

                s = new_string;
            },
            _ => {},
        }
    }
}
