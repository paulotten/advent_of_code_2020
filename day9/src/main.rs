mod data;

fn main() {
    let data = data::get_data();
    let numbers: Vec<u64> = data.lines().map(|x| x.parse().unwrap()).collect();

    _puzzle2(numbers);
}

fn _puzzle2(numbers: Vec<u64>) {
    let target = 41682220; // from puzzle 1

    for i in 0..numbers.len() {
        let mut sum = numbers[i];
        let mut min = numbers[i];
        let mut max = numbers[i];

        for j in i+1..numbers.len() {
            sum += numbers[j];

            if numbers[j] < min {
                min = numbers[j];
            }
            if numbers[j] > max {
                max = numbers[j];
            }

            if sum == target {
                println!("{}", min + max);
                return;
            } else if sum > target {
                break;
            }
        }
    }
}

fn _puzzle1(numbers: Vec<u64>) {
    const OFFSET: usize = 25;

    for i in OFFSET..numbers.len() {
        let mut valid = false;

        'j: for j in i-OFFSET..i {
            for k in i-OFFSET..i {
                // no using the same number twice
                if j == k {
                    continue;
                }

                if numbers[j] + numbers[k] == numbers[i] {
                    valid = true;
                    break 'j;
                }
            }
        }

        if !valid {
            println!("{}", numbers[i]); // 41682220
            return;
        }
    }
}
