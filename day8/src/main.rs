mod data;

use std::collections::HashSet;

struct Instruction {
    op: &'static str,
    arg: isize,
}

impl Instruction {
    fn parse(s: &'static str) -> Instruction {
        let parts: Vec<_> = s.split(' ').collect();

        Instruction {
            op: parts[0],
            arg: parts[1].parse().unwrap(),
        }
    }
}

fn main() {
    //let data = data::_get_sample_data();
    let data = data::get_data();
    let instructions = data.lines().map(Instruction::parse).collect();

    _puzzle1(instructions);
}

fn _puzzle1(instructions: Vec<Instruction>) {
    let mut visited_lines: HashSet<usize> = HashSet::new();
    let mut accumulator: isize = 0;
    let mut i: usize = 0;

    loop {
        if visited_lines.contains(&i) {
            break;
        }
        visited_lines.insert(i);

        match (instructions[i].op, instructions[i].arg) {
            ("nop", _) => i += 1,
            ("acc", arg @ _) => {
                accumulator += arg;
                i += 1;
            }
            ("jmp", arg @ _) => i = (i as isize + arg) as usize,
            (op @ _, _) => panic!("unknown opcode: {}", op),
        };
    }

    println!("{}", accumulator);
}
