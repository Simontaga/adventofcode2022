// For each char
// If that char + 3 next ones are unique.
// return index of last char needed (+1)

pub fn day6_part_1() -> i32 {
    let data = get_input();

    let mut ans = 0;
    let mut unique: Vec<char> = Vec::new();
    let lookup_copy: Vec<_> = data.chars().collect();


    for (i, _) in data.chars().into_iter().enumerate() {
        unique.push(*lookup_copy.get(i).unwrap());
        unique.push(*lookup_copy.get(i + 1).unwrap());
        unique.push(*lookup_copy.get(i + 2).unwrap());
        unique.push(*lookup_copy.get(i + 3).unwrap());
        
        // Values need to be sorted in order to dedupe..
        unique.sort();
        unique.dedup();

        // If we still have 4 chars after deduping, we got the answer.
        if unique.len() == 4 {
            ans = i+4;
            break;
        }
        unique.clear();
    }

    ans as i32
}

fn get_input() -> String {
    std::fs::read_to_string("src/days/input/6.txt").unwrap()
}
