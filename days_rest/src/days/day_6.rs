use std::collections::HashSet;

pub fn day6_part_1() -> i16 { get_index_unique(4) }

pub fn day6_part_2() -> i16 { get_index_unique(14) }

fn get_input() -> String { std::fs::read_to_string("src/days/input/6.txt").unwrap() }


fn get_index_unique(target: usize) -> i16 {
    let characters: Vec<char> = get_input().chars().collect();

    for i in 0..characters.len() {
        let window:HashSet<char> = HashSet::from_iter(characters[i..i + target].to_vec());
        if window.len() != target { continue; }
        return (i + target) as i16
    }

    0
}

