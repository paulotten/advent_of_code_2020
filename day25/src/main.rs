fn main() {
    let pub_a = 9033205;
    let pub_b = 9281649;

    // sample
    //let pub_a = 5764801;
    //let pub_b = 17807724;

    part1(pub_a, pub_b);
}

fn part1(pub_a: u64, pub_b: u64) {
    let mut value = 1;
    let pub_x;
    let mut loop_size = 0;

    loop {
        if value == pub_a {
            pub_x = pub_b;
            break;
        } else if value == pub_b {
            pub_x = pub_a;
            break;
        }

        loop_size += 1;

        value = crypto_loop(value, 7);
    }

    println!("{} {}", pub_x, loop_size);

    value = 1;

    for _ in 0..loop_size {
        value = crypto_loop(value, pub_x);
    }

    println!("{}", value);
}

fn crypto_loop(mut value: u64, subject: u64) -> u64 {
    value *= subject;
    value %= 20201227;

    value
}
