mod data;

use std::collections::HashSet;

#[derive(Clone)]
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

    _puzzle2(instructions);
}

fn _puzzle2(instructions: Vec<Instruction>) {
    for i in 0..instructions.len() {
        let op = instructions[i].op;

        if op == "nop" || op == "jmp" {
            let mut clone = instructions.clone();

            clone[i].op = match op {
                "nop" => "jmp",
                "jmp" => "nop",
                _ => panic!(),
            };

            let result = run(clone);

            if result.is_ok() {
                println!("{:?}", result.unwrap());
            }
        }
    }
}

fn _puzzle1(instructions: Vec<Instruction>) {
    println!("{:?}", run(instructions));
}

fn run(instructions: Vec<Instruction>) -> Result<isize, isize> {
    let mut visited_lines: HashSet<usize> = HashSet::new();
    let mut accumulator: isize = 0;
    let mut i: usize = 0;

    loop {
        if i >= instructions.len() {
            return Ok(accumulator);
        }

        if visited_lines.contains(&i) {
            return Err(accumulator);
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
}
