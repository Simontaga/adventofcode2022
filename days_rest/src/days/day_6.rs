pub fn day6_part_1() -> i32 {
    let data = get_input();
    return get_index_unique(data, 4);
}

fn get_index_unique(string: String, target: usize) -> i32 {
    let mut ans = 0;
    let mut unique: Vec<char> = Vec::new();
    let lookup_copy: Vec<char> = string.chars().collect();

    for (i, _) in string.chars().into_iter().enumerate() {
        for k in 0..target {
            unique.push(*lookup_copy.get(i + k).unwrap());
        }
        
        unique.sort();
        unique.dedup();

        if unique.len() == target {
            ans = i+target;
            break;
        }
        unique.clear();
    }

    ans as i32
}


pub fn day6_part_2() -> i32 {
    let data = get_input();
    return get_index_unique(data, 14);
}

fn get_input() -> String {
    std::fs::read_to_string("src/days/input/6.txt").unwrap()
}
