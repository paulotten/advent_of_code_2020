fn main() {
    // puzzle data
    let data = "137826495";
    let num_moves = 100;

    // sample data
    //let data = "389125467";
    //let num_moves = 10;

    let mut numbers: Vec<u8> = data.chars().map(|c| String::from(c).parse().unwrap()).collect();

    part1(&mut numbers, num_moves);
}

fn part1(numbers: &mut Vec<u8>, num_moves: u8) {
    let num_numbers = numbers.len() as u8;

    for i in 1..=num_moves {
        println!("-- move {} --", i);
        println!("cups: {:?}", numbers);

        let current = numbers[0];
        //println!("currect: {}", current);

        let pickup = numbers[1..4].to_vec();
        println!("pick up: {:?}", pickup);

        *numbers = numbers[4..].to_vec();
        //println!("remaining: {:?}", numbers);

        let mut destination = current - 1;
        loop {
            if destination == 0 {
                destination = num_numbers;
            }

            if numbers.contains(&destination) {
                break;
            }

            destination -= 1;
        }
        println!("destination: {}", destination);

        let mut destination_index = numbers.iter().position(|x| *x == destination).unwrap();

        // re-insert pickup
        for number in pickup {
            destination_index += 1;
            numbers.insert(destination_index, number);
        }

        // add current to end
        numbers.push(current);

        println!();
    }

    println!("-- final --");
    println!("cups: {:?}", numbers);
}
