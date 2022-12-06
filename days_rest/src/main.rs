use std::time::{ Instant };
mod days;

fn main() {
    let now = Instant::now();
    println!("Day 3 Part 1:{}", days::day_3::day_3_part_1());
    println!("Took:{}ms", now.elapsed().as_millis());

    let now = Instant::now();
    println!("Day 3 Part 2: {}", days::day_3::day3_part_2());
    println!("Took:{}ms", now.elapsed().as_millis());

    let now = Instant::now();
    println!("Day 4 Part 1: {}", days::day_4::day4_part_1());
    println!("Took:{}ms", now.elapsed().as_millis());

    let now = Instant::now();
    println!("Day 4 Part 2: {}", days::day_4::day4_part_2());
    println!("Took:{}ms", now.elapsed().as_millis());

    let now = Instant::now();
    println!("Day 5 Part 1: {}", days::day_5::day5_part_1());
    println!("Took:{}ms", now.elapsed().as_millis());

    let now = Instant::now();
    println!("Day 5 Part 2: {}", days::day_5::day5_part_2());
    println!("Took:{}ms", now.elapsed().as_millis());

    let now = Instant::now();
    println!("Day 6 Part 1: {}", days::day_6::day6_part_1());
    println!("Took:{}ms", now.elapsed().as_millis());
}
