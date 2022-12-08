use std::thread::Thread;
use std::{thread, time};

#[derive(Debug, Copy, Clone)]
struct Tree {
    x: u32,
    y: u32,
    height: u32,
}

pub fn day8_part_1() -> i32 {
    let input = get_input();
    let lines: Vec<&str> = input.lines().collect();
    let mut trees: Vec<Tree> = Vec::new();
    let max_y = lines.len() - 1;
    let mut max_x = 0;

    for (y, line) in lines.into_iter().enumerate() {
        for (x, height) in line.chars().enumerate() {
            if max_x < x {
                max_x = x;
            }
            trees.push(Tree {
                x: x as u32,
                y: y as u32,
                height: height.to_digit(10).unwrap(),
            })
        }
    }
    trees.reverse();
    count_trees_visible(trees, max_y as u32, max_x as u32)
}

// Extremely inefficent, not blazingly fast.
fn count_trees_visible(trees: Vec<Tree>, max_y: u32, max_x: u32) -> i32 {
    let trees_clone = trees.clone();
    let mut score = 0;

    for tree in trees {
        // All trees higher or equal than the current one, to the right
        let right: Vec<&Tree> = trees_clone
            .iter()
            .filter(|x| x.height >= tree.height && x.x > tree.x && x.y == tree.y)
            .collect();



        // All trees higher or equal than the current one, to the left
        let left: Vec<&Tree> = trees_clone
            .iter()
            .filter(|x| x.height >= tree.height && x.x < tree.x && x.y == tree.y)
            .collect();


        // All trees higher or equal than the current one, upwards
        let down: Vec<&Tree> = trees_clone
            .iter()
            .filter(|x| x.height >= tree.height && x.y > tree.y && x.x == tree.x)
            .collect();

        


        // All trees higher or equal than the current one, downwards
        let up: Vec<&Tree> = trees_clone
            .iter()
            .filter(|x| x.height >= tree.height && x.y < tree.y && x.x == tree.x)
            .collect();


        if up.len() == 0 || down.len() == 0 || left.len() == 0 || right.len() == 0 {
            score += 1;
        }
    }

    score
}

fn get_input() -> String {
    std::fs::read_to_string("src/days/input/8.txt").unwrap()
}

/* 

            println!("Current tree {}", tree.height);
            for r in &up {
                print!("{}", r.height);
            }
            println!("{}", "");
            println!("Len {}", up.len());
            
    
            let ten_millis = time::Duration::from_millis(1000000);
            let now = time::Instant::now();
    
            thread::sleep(ten_millis); */