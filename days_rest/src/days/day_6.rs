pub fn day6_part_1() -> i32 {
    get_index_unique(4)
}

pub fn day6_part_2() -> i32 {
    get_index_unique(14)
}

fn get_index_unique(target: usize) -> i32 {
    let characters: Vec<char> = get_input().chars().collect();

    for i in 0..characters.len() {
        let mut window: Vec<char> = characters[i..i + target].to_vec();
        window.sort_unstable();
        window.dedup();
        if window.len() != target { continue; }
        return (i + target) as i32;
    }

    0
}

fn get_input() -> String {
    std::fs::read_to_string("src/days/input/6.txt").unwrap()
}
