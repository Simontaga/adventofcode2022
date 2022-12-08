
use take_until::TakeUntilExt;


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
    let lines = input.lines();
    let mut trees: Vec<Tree> = Vec::new();

    for (y, line) in lines.collect::<Vec<&str>>().into_iter().enumerate() {
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
fn count_trees_visible(trees: Vec<Tree>, _max_y: u32, _max_x: u32) -> i32 {
    let trees_clone = trees.clone();
    let mut score = 0;

    for tree in trees {
        // All trees higher or equal than the current one, to the right
        let right = trees_clone
            .iter()
            .filter(|x| x.height >= tree.height && x.x > tree.x && x.y == tree.y);

        // All trees higher or equal than the current one, to the left
        let left = trees_clone
            .iter()
            .filter(|x| x.height >= tree.height && x.x < tree.x && x.y == tree.y);

        // All trees higher or equal than the current one, upwards
        let down = trees_clone
            .iter()
            .filter(|x| x.height >= tree.height && x.y > tree.y && x.x == tree.x);

        // All trees higher or equal than the current one, downwards
        let up = trees_clone
            .iter()
            .filter(|x| x.height >= tree.height && x.y < tree.y && x.x == tree.x);

        if up.count() == 0 || down.count() == 0 || left.count() == 0|| right.count() == 0 {
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
        let right = trees_clone
            .iter()
            .filter(|x| x.x > tree.x && x.y == tree.y)
            .take_until(|x| x.height >= tree.height);

        let left = trees_clone
            .iter()
            .filter(|x| x.x < tree.x && x.y == tree.y)
            .rev()
            .take_until(|x| x.height >= tree.height);

        let down = trees_clone
            .iter()
            .filter(|x| x.y > tree.y && x.x == tree.x)
            .take_until(|x| x.height >= tree.height);


        let up = trees_clone
            .iter()
            .filter(|x| x.y < tree.y && x.x == tree.x)
            .rev()
            .take_until(|x| x.height >= tree.height);

        let scenic_score = right.count() * left.count() * down.count() * up.count();

        if scenic_score > score {
            score = scenic_score;
        }
    }

    score as i32
}

fn get_input() -> String {
    std::fs::read_to_string("src/days/input/8.txt").unwrap()
}