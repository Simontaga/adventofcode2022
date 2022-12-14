use std::collections::HashSet;

pub fn day6_part_1() -> i16 { get_index_unique(4) }

pub fn day6_part_2() -> i16 { get_index_unique(14) }

fn get_input() -> String { std::fs::read_to_string("src/days/input/6.txt").unwrap() }


fn get_index_unique(target: usize) -> i16 {
    for (i, c) in get_input().chars().collect::<Vec<char>>().windows(target).enumerate() {
        if HashSet::<char>::from_iter(c.to_vec()).len() == target { return (i + target) as i16; }
    }
    0
}

