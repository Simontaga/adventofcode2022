use std::thread::Thread;
use take_until::TakeUntilExt;
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

pub fn day8_part_2() -> i32 {
    let input = get_input();
    let lines: Vec<&str> = input.lines().collect();
    let mut trees: Vec<Tree> = Vec::new();

    for (y, line) in lines.into_iter().enumerate() {
        for (x, height) in line.chars().enumerate() {
            trees.push(Tree {
                x: x as u32,
                y: y as u32,
                height: height.to_digit(10).unwrap(),
            })
        }
    }
    
    count_trees_scenic_score(trees)
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

fn count_trees_scenic_score(trees: Vec<Tree>) -> i32 {
    let trees_clone = trees.clone();
    let mut score = 0;

    for tree in trees {
        // Trees to the right at same Y until height is more or the same
        let right: Vec<&Tree> = trees_clone
            .iter()
            .filter(|x| x.x > tree.x && x.y == tree.y)
            .take_until(|x| x.height >= tree.height)
            .collect();

        let left: Vec<&Tree> = trees_clone
            .iter()
            .filter(|x| x.x < tree.x && x.y == tree.y)
            .rev()
            .take_until(|x| x.height >= tree.height)
            .collect();

        let down: Vec<&Tree> = trees_clone
            .iter()
            .filter(|x| x.y > tree.y && x.x == tree.x)
            .take_until(|x| x.height >= tree.height)
            .collect();

        let up: Vec<&Tree> = trees_clone
            .iter()
            .filter(|x| x.y < tree.y && x.x == tree.x)
            .rev()
            .take_until(|x| x.height >= tree.height)
            .collect();

        let scenic_score = right.len() * left.len() * down.len() * up.len();

        if scenic_score > score {
            score = scenic_score;
        }
    }

    score as i32
}

fn get_input() -> String {
    std::fs::read_to_string("src/days/input/8.txt").unwrap()
}