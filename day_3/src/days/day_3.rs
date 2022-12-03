use lazy_static::lazy_static;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


#[derive(Clone)]
struct Rucksack {
    compartment_1 : Compartment,
    compartment_2 : Compartment
}

#[derive(Clone)]
struct Compartment {
    contents: String
}


struct Group {
    ruck_1: Rucksack,
    ruck_2: Rucksack,
    ruck_3: Rucksack,
}
lazy_static! {
    static ref ALPHABET: HashMap<&'static str, i32> = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
        ("d", 4),
        ("e", 5),
        ("f", 6),
        ("g", 7),
        ("h", 8),
        ("i", 9),
        ("j", 10),
        ("k", 11),
        ("l", 12),
        ("m", 13),
        ("n", 14),
        ("o", 15),
        ("p", 16),
        ("q", 17),
        ("r", 18),
        ("s", 19),
        ("t", 20),
        ("u", 21),
        ("v", 22),
        ("w", 23),
        ("x", 24),
        ("y", 25),
        ("z", 26)
    ]);
}



pub fn day_3_part_1 () -> i32 {
   
    let rucksack = get_rucksacks();
    let mut score: i32 = 0;

    for ruck in rucksack {
        score += calculate_rucksack_score(ruck);
    }

    score
}

pub fn day3_part_2() -> i32 {
    let mut ruck_sacks = get_rucksacks();
    let mut score = 0;

    loop {
        match get_group(&mut ruck_sacks) {
            Some(group) => {
                score += calculate_group_score(&group);
            }

            None => {
                return score
            }
        }

    }
}

// Truly a beauty 😎
fn calculate_group_score (group: &Group) -> i32 {
    let group_1_string = group.ruck_1.compartment_1.contents.to_string() + &group.ruck_1.compartment_2.contents.to_string();
    let group_2_string = group.ruck_2.compartment_1.contents.to_string() + &group.ruck_2.compartment_2.contents.to_string();
    let group_3_string = group.ruck_3.compartment_1.contents.to_string() + &group.ruck_3.compartment_2.contents.to_string();

    for g1 in group_1_string.chars().into_iter() {  
        if char::is_whitespace(g1) { continue; }

        for g2 in group_2_string.chars().into_iter() {
            if g1 != g2 { continue; }

            for g3 in group_3_string.chars().into_iter()  {
                if g2 != g3 { continue; }
                return get_letter_score(&g1.to_string());
            }
        }
    }

    0
}

// Mm yes.
fn get_group(ruck_sacks : &mut Vec<Rucksack>) -> Option<Group> {
    match ruck_sacks.len() >= 3 {
        true => {
            let elves_1 = &ruck_sacks.pop().unwrap();
            let elves_2 = &ruck_sacks.pop().unwrap();
            let elves_3 = &ruck_sacks.pop().unwrap();
            
            let group = Group {
                ruck_1 : elves_1.to_owned(),
                ruck_2 : elves_2.to_owned(),
                ruck_3 : elves_3.to_owned(),
            };

            Some(group)
        }

        _ => {
            None
        }
    }
}



// Very nice
fn calculate_rucksack_score (ruck_sack : Rucksack) -> i32 {
    let compartment_1_letters: Vec<&str> = ruck_sack.compartment_1.contents.split("").collect();
    let compartment_2_letters: Vec<&str> = ruck_sack.compartment_2.contents.split("").collect();


   for c1 in &compartment_1_letters {
        for c2 in &compartment_2_letters {

            if c1 == c2 && c1 != &"" {
                return get_letter_score(c1);
            }
        }
    } 

    0
}



fn get_letter_score (letter: &str) -> i32 {
    // Purely genius
    match ALPHABET.contains_key(&letter) {
        true => {
            return *ALPHABET.get(*&letter).unwrap();
        },
        false => {
            let lower = letter.to_lowercase();
            return ALPHABET[&lower[..]] + 26 as i32;
        }
    }
}


fn get_rucksacks () -> Vec<Rucksack> {
    let data: String = get_input();
    let lines = data.split("\n");

    let mut ruck_sack: Vec<Rucksack> = Vec::new();

    for rucksack in lines {
        let lenght = rucksack.len();


        let first = Compartment {
            contents: rucksack[0..lenght/2].to_string()
        };

        let second = Compartment {
            contents: rucksack[lenght/2..].to_string()
        };


        let ruck: Rucksack = Rucksack {
            compartment_1: first,
            compartment_2: second
        };

        ruck_sack.push(ruck);
    }

    return ruck_sack;
}

fn get_input () -> String {
    let mut file = File::open("src/days/input/3.txt").unwrap();

    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();

    buffer
}
