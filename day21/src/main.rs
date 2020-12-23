mod data;

use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Debug)]
struct Food {
    ingredients: HashSet<&'static str>,
    allergens: HashSet<&'static str>,
}

impl Food {
    fn parse(s: &'static str) -> Food {
        let parts: Vec<_> = s.split(" (contains ").collect();
        assert!(parts.len() == 2);

        let ingredients = parts[0].split(' ').collect();

        let parts: Vec<_> = parts[1].split(")").collect();
        let allergens = parts[0].split(", ").collect();

        Food {
            ingredients,
            allergens,
        }
    }
}

fn main() {
    let data = data::get_data();
    //let data = data::_sample();

    let foods: Vec<_> = data.lines().map(Food::parse).collect();

    part2(&foods);
}

fn _part1(foods: &Vec<Food>) {
    let mut all_ingredients = vec![];

    let mut allergen_possible_ingredients: HashMap<&str, HashSet<&str>> = HashMap::new();

    for food in foods {
        for ing in &food.ingredients {
            all_ingredients.push(ing);
        }

        for al in &food.allergens {
            if allergen_possible_ingredients.contains_key(al) {
                let updated_possible_ingredients = allergen_possible_ingredients
                    .get(al)
                    .unwrap()
                    .intersection(&food.ingredients)
                    .map(|x| *x)
                    .collect();

                allergen_possible_ingredients.insert(al, updated_possible_ingredients);
            } else {
                allergen_possible_ingredients.insert(al, food.ingredients.clone());
            }
        }
    }

    let allergen_possible_ingredients: HashSet<_> =
        allergen_possible_ingredients.values().flatten().collect();

    println!(
        "{}",
        all_ingredients
            .iter()
            .filter(|x| !allergen_possible_ingredients.contains(*x))
            .count()
    );
}

fn part2(foods: &Vec<Food>) {
    let mut allergen_possible_ingredients: HashMap<&str, HashSet<&str>> = HashMap::new();

    for food in foods {
        for al in &food.allergens {
            if allergen_possible_ingredients.contains_key(al) {
                let updated_possible_ingredients = allergen_possible_ingredients
                    .get(al)
                    .unwrap()
                    .intersection(&food.ingredients)
                    .map(|x| *x)
                    .collect();

                allergen_possible_ingredients.insert(al, updated_possible_ingredients);
            } else {
                allergen_possible_ingredients.insert(al, food.ingredients.clone());
            }
        }
    }

    // recursivly isolate allergens to individual ingredients
    let mut found = true;
    let mut isolated_allergens = BTreeMap::new();
    let mut used_ingredients = HashSet::new();

    while found {
        found = false;

        for (al, ingredients) in &allergen_possible_ingredients {
            let difference: Vec<_> = ingredients.difference(&used_ingredients).collect();

            if difference.len() == 1 {
                let ingredient = difference[0].clone();

                isolated_allergens.insert(al, ingredient);
                used_ingredients.insert(ingredient);

                found = true;
            }
        }
    }

    println!("{}", isolated_allergens.values().map(|x| *x).collect::<Vec<_>>().join(","));
}
