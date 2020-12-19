mod data;

fn main() {
    let data = data::get_data();
    //let data = data::_sample_data();

    puzzle2(data.lines());
}

fn _puzzle1(lines: std::str::Lines) {
    let sum: i64 = lines.map(|l| _solve(String::from(l))).sum();

    println!("{}", sum);
}

fn puzzle2(lines: std::str::Lines) {
    let sum: i64 = lines.map(|l| solve2(String::from(l))).sum();

    println!("{}", sum);
}

fn _solve(mut eq: String) -> i64 {
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
        let solved = _solve(String::from(without_brackets));

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

fn solve2(mut eq: String) -> i64 {
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
        let solved = solve2(String::from(without_brackets));

        eq = eq.replace(with_brackets, &solved.to_string());
    }

    // equation solver
    eq = apply_operation(eq, '+', &|a,b| a+b);
    eq = apply_operation(eq, '*', &|a,b| a*b);

    let result = eq.parse().unwrap();

    result
}

fn apply_operation(mut eq: String, op_symbol: char, op_function: &dyn Fn(i64, i64) -> i64) -> String {
    let search = format!(" {} ", op_symbol);

    loop {
        let maybe_match = eq.find(&search);

        if maybe_match.is_none() {
            break;
        }

        let m = maybe_match.unwrap();

        let start = match eq[0..m].rfind(' ') {
            Some(x) => x + 1,
            None => 0,
        };

        let offset = search.len();
        let tail = &eq[m+offset..];

        let end = m + offset + tail.find(' ').unwrap_or(tail.len());

        let before = &eq[start..m];
        let after = &eq[m+offset..end];

        let a: i64 = before.parse().unwrap();
        let b: i64 = after.parse().unwrap();

        let result = op_function(a, b);
        
        // have to use a range based replacement here
        // otherwise end up replacing partial leading/trailing numbers
        eq.replace_range(start..end, &result.to_string());
    }

    eq
}
