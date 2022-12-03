use lazy_static::lazy_static;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::thread;


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
    let ruck_sacks = get_rucksacks();
    let mut first_half = ruck_sacks[..ruck_sacks.len()/2].to_vec();
    let mut second_half = ruck_sacks[ruck_sacks.len()/2..].to_vec();

    
    let thread_1 = thread::spawn(move || {
        let mut score = 0;

        loop {
            match get_group(&mut first_half) {
                Some(group) => {
                    score += calculate_group_score(&group);
                }
    
                None => {
                    return score
                }
            } }
    });

    let thread_2 = thread::spawn(move || {
        let mut score = 0;

        loop {
            match get_group(&mut second_half) {
                Some(group) => {
                    score += calculate_group_score(&group);
                }
    
                None => {
                    return score
                }
            } }
    });

    let thread_1_score = thread_1.join().unwrap();
    let thread_2_score = thread_2.join().unwrap();
    return thread_1_score + thread_2_score
}

// Truly a beauty 😎
fn calculate_group_score (group: &Group) -> i32 {
    let group_1_string = group.ruck_1.compartment_1.contents.to_string() + &group.ruck_1.compartment_2.contents.to_string();
    let group_2_string = group.ruck_2.compartment_1.contents.to_string() + &group.ruck_2.compartment_2.contents.to_string();
    let group_3_string = group.ruck_3.compartment_1.contents.to_string() + &group.ruck_3.compartment_2.contents.to_string();

    for g1 in group_1_string.chars() {  
        if char::is_whitespace(g1) { continue; }

        for g2 in group_2_string.chars() {
            if g1 != g2 { continue; }

            for g3 in group_3_string.chars()  {
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
            let group: Group = Group {
                ruck_1 : ruck_sacks.pop().unwrap(),
                ruck_2 : ruck_sacks.pop().unwrap(),
                ruck_3 : ruck_sacks.pop().unwrap(),
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
   for c1 in ruck_sack.compartment_1.contents.chars() {
        if char::is_whitespace(c1) { continue; }
        for c2 in ruck_sack.compartment_2.contents.chars() {
            if c1 != c2 { continue; }
            return get_letter_score(&c1.to_string());            
        }
    } 

    0
}


fn get_letter_score (letter: &str) -> i32 {
    match ALPHABET.contains_key(&letter) {
        true => {
            return *ALPHABET.get(letter).unwrap();
        },
        false => {
            let lower = letter.to_lowercase();
            ALPHABET[&lower[..]] + 26_i32
        }
    }
}


fn get_rucksacks () -> Vec<Rucksack> {
    let data: String = get_input();
    let lines = data.split('\n');

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

    ruck_sack
}

fn get_input () -> String {
    let mut file = File::open("src/days/input/3.txt").unwrap();

    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();

    buffer
}
