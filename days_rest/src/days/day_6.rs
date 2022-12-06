pub fn day6_part_1() -> i32 {
    let data = get_input();
    get_index_unique(data, 4)
}

fn get_index_unique(string: String, target: usize) -> i32 {
    let mut unique: Vec<char> = Vec::new();
    let characters: Vec<char> = string.chars().collect();

    for i in 0..characters.len() {
        for k in 0..target {
            unique.push(*characters.get(i + k).unwrap());
        }

        unique.sort_unstable();
        unique.dedup();

        if unique.len() == target {
            return (i + target) as i32;
        }
        unique.clear();
    }

    0
}

pub fn day6_part_2() -> i32 {
    let data = get_input();
    get_index_unique(data, 14)
}

fn get_input() -> String {
    std::fs::read_to_string("src/days/input/6.txt").unwrap()
}
