mod data;

fn main() {
    let data = data::get_data();
    //let data = data::_sample();

    let mut decks: Vec<Vec<_>> = data
        .split("\n\n")
        .map(|d| {
            d.lines()
                .skip(1)
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    
    assert!(decks.len() == 2);

    part2(&mut decks);
}

fn _part1(decks: &mut Vec<Vec<usize>>) {
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

    let winning_deck: Vec<_> = decks.iter().flatten().map(|x| *x).collect();

    println!("{}", get_score(&winning_deck));
}

fn part2(decks: &mut Vec<Vec<usize>>) {
    let decks = recurse(decks);

    let winning_deck: Vec<_> = decks.iter().flatten().map(|x| *x).collect();

    println!("{}", get_score(&winning_deck));
}

fn recurse(decks: &mut Vec<Vec<usize>>) -> &Vec<Vec<usize>> {
    let mut previous_decks = vec![];

    while decks[0].len() > 0 && decks[1].len() > 0 {
        if previous_decks.contains(decks) {
            decks[1] = vec![];

            return decks;
        }
        previous_decks.push(decks.clone());

        let card0 = decks[0].remove(0);
        let card1 = decks[1].remove(0);

        let winner;

        if decks[0].len() >= card0 && decks[1].len() >= card1 {
            let mut sub_decks = vec![];

            sub_decks.push(decks[0][0..card0].to_vec());
            sub_decks.push(decks[1][0..card1].to_vec());
            
            let sub_decks = recurse(&mut sub_decks);

            if sub_decks[0].len() > 0 {
                winner = 0;
            } else {
                winner = 1;
            }
        } else {
            if card0 > card1 {
                winner = 0;
            } else {
                winner = 1;
            }
        }

        if winner == 0 {
            decks[0].push(card0);
            decks[0].push(card1);
        } else {
            decks[1].push(card1);
            decks[1].push(card0);
        }
    }

    decks
}

fn get_score(deck: &Vec<usize>) -> usize {
    let mut sum = 0;
    let mut mult = 1;

    for card in deck.iter().rev() {
        sum += card*mult;
        mult += 1;
    }

    sum
}
