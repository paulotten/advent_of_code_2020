mod data;

fn main() {
    let data = data::get_data();
    //let data = data::_sample();

    let mut decks: Vec<Vec<_>> = data
        .split("\n\n")
        .map(|d| {
            d.lines()
                .skip(1)
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    
    assert!(decks.len() == 2);

    part1(&mut decks);
}

fn part1(decks: &mut Vec<Vec<u32>>) {
    while decks[0].len() > 0 && decks[1].len() > 0 {
        let card0 = decks[0].remove(0);
        let card1 = decks[1].remove(0);

        if card0 > card1 {
            decks[0].push(card0);
            decks[0].push(card1);
        } else {
            decks[1].push(card1);
            decks[1].push(card0);
        }
    }

    let winning_deck: Vec<_> = decks.iter().flatten().collect();

    let mut sum = 0;
    let mut mult = 1;

    for card in winning_deck.iter().rev() {
        sum += *card*mult;
        mult += 1;
    }

    println!("{}", sum);
}
