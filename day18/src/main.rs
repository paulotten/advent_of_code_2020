mod data;

fn main() {
    let data = data::get_data();
    //let data = data::_sample_data();

    puzzle1(data.lines());
}

fn puzzle1(lines: std::str::Lines) {
    let sum: i64 = lines.map(|l| solve(String::from(l))).sum();

    println!("{}", sum);
}

fn solve(mut eq: String) -> i64 {
    // brackets preprocessor
    loop {
        let maybe_bracket = eq.find(')');

        if maybe_bracket.is_none() {
            break;
        }

        let end = maybe_bracket.unwrap() + 1;
        let start = eq[0..end].rfind('(').unwrap();

        let with_brackets = &eq[start..end];
        let without_brackets = &eq[start + 1..end - 1];
        let solved = solve(String::from(without_brackets));

        eq = eq.replace(with_brackets, &solved.to_string());
    }

    // equation solver
    let mut operation = Some("+");
    let mut result = 0;

    for part in eq.split(" ") {
        if operation.is_some() {
            let value: i64 = part.parse().unwrap();

            match operation.unwrap() {
                "+" => result += value,
                "*" => result *= value,
                op @ _ => panic!("unsupported operation: {}", op),
            }

            operation = None;
        } else {
            operation = Some(part);
        }
    }

    result
}
