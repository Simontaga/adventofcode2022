use std::str::FromStr;


struct Pair {
    range1_start: i32,
    range2_start: i32,
    range1_end: i32,
    range2_end: i32
}

pub fn day4_part_1 () -> i32 {
    let data: String = get_input();
    let lines = data.lines();
    let mut collisions = 0;
    
    let mut pairs: Vec<Pair> = Vec::new();

    for line in lines {
        pairs.push(get_pair(line));
    } 
    
    for pair in pairs {
        if pair.range1_start >= pair.range2_start && pair.range1_end <= pair.range2_end
            || pair.range2_start >= pair.range1_start && pair.range2_end <= pair.range1_end {
                collisions += 1;
            }
     }

     collisions
}

pub fn day4_part_2() -> i32 {
    let data: String = get_input();
    let lines = data.split('\n');
    let mut overlaps = 0;
    
    let mut pairs: Vec<Pair> = Vec::new();

    for line in lines {
        pairs.push(get_pair(line));
    } 

    for pair in pairs {
        if pair.range1_start <= pair.range2_end && pair.range2_start <= pair.range1_end {
            overlaps += 1
        }
    }

    overlaps
}

fn get_pair(line: &str) -> Pair {
    let split = line.split(',');
    let pairs: Vec<&str> = split.collect();

    let mut nums: Vec<i32> = Vec::new();

    for pair in pairs {
        let split_pair: Vec<&str> = pair.split('-').collect();
        nums.push(FromStr::from_str(split_pair[0]).unwrap());
        nums.push(FromStr::from_str(split_pair[1]).unwrap()); 
    }
    
    Pair { range1_start: nums[0], range1_end: nums[1], range2_start: nums[2], range2_end: nums[3] }
}

fn get_input () -> String {
    std::fs::read_to_string("src/days/input/4.txt").unwrap()
}