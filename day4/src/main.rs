mod data;

use data::Passport;

fn main() {
    let passports = data::get_passports();

    puzzle2(passports);
}

fn _puzzle1(passports: Vec<Passport>) {
    let mut valid = 0;

    for passport in passports {
        if passport.contains_key("byr")
            && passport.contains_key("iyr")
            && passport.contains_key("eyr")
            && passport.contains_key("hgt")
            && passport.contains_key("hcl")
            && passport.contains_key("ecl")
            && passport.contains_key("pid")
        {
            valid += 1;
        }
    }

    println!("{}", valid);
}

fn puzzle2(passports: Vec<Passport>) {
    let mut valid = 0;

    for passport in passports {
        if passport_is_valid(passport) {
            valid += 1;
        }
    }

    println!("{}", valid);
}

fn passport_is_valid(passport: Passport) -> bool {
    if !(passport.contains_key("byr")
        && passport.contains_key("iyr")
        && passport.contains_key("eyr")
        && passport.contains_key("hgt")
        && passport.contains_key("hcl")
        && passport.contains_key("ecl")
        && passport.contains_key("pid"))
    {
        return false;
    }

    if !valid_range(passport.get("byr").unwrap(), 1920, 2002) {
        return false;
    }
    if !valid_range(passport.get("iyr").unwrap(), 2010, 2020) {
        return false;
    }
    if !valid_range(passport.get("eyr").unwrap(), 2020, 2030) {
        return false;
    }

    // hgt
    let hgt = passport.get("hgt").unwrap();
    let len = hgt.len();
    if len < 3 {
        return false;
    }
    match &hgt[len-2..] {
        "cm" => {
            if !valid_range(&hgt[..len-2], 150, 193) {
                return false;
            }
        },
        "in" => {
            if !valid_range(&hgt[..len-2], 59, 76) {
                return false;
            }
        },
        _ => return false,
    }

    // hcl
    let hcl = passport.get("hcl").unwrap();
    if hcl.len() != 7 {
        return false;
    }
    let hcl: Vec<char> = hcl.chars().collect();
    if hcl[0] != '#' {
        return false;
    }
    for i in 1..=6 {
        if !valid_hex(hcl[i]) {
            return false;
        }
    }

    // ecl
    let ecl = *passport.get("ecl").unwrap();
    match ecl {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {},
        _ => return false,
    }

    // pid
    let pid = passport.get("pid").unwrap();
    if pid.len() != 9 {
        return false;
    }
    let maybe_int: Result<i32, _> = pid.parse();
    if maybe_int.is_err() {
        return false;
    }

    true
}

fn valid_range(number: &str, min: i32, max: i32) -> bool {
    let maybe_int: Result<i32, _> = number.parse();

    if maybe_int.is_err() {
        return false;
    }

    let number = maybe_int.unwrap();

    number >= min && number <= max
}

fn valid_hex(digit: char) -> bool {
    match digit {
        '0'..='9' => true,
        'a'..='f' => true,
        _ => false,
    }
}
