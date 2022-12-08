use take_until::TakeUntilExt;

#[derive(Debug, Copy, Clone)]
struct Tree {
    x: u32,
    y: u32,
    height: u32,
}

pub fn day8_part_1() -> i32 {
    let input = get_input();
    let trees: Vec<Tree> = get_trees(input);
    count_trees_visible(trees)
}

pub fn day8_part_2() -> i32 {
    let input = get_input();
    let trees: Vec<Tree> = get_trees(input);
    count_trees_scenic_score(trees)
}

fn get_trees(input: String) -> Vec<Tree> {
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

    trees
}

fn count_trees_visible(trees: Vec<Tree>) -> i32 {
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

        if up.count() == 0 || down.count() == 0 || left.count() == 0 || right.count() == 0 {
            score += 1;
        }
    }

    score
}

fn count_trees_scenic_score(trees: Vec<Tree>) -> i32 {
    let trees_clone = trees.clone();
    let mut score = 0;

    for tree in trees {

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

        if scenic_score > score { score = scenic_score; }
    }

    score as i32
}

fn get_input() -> String {
    std::fs::read_to_string("src/days/input/8.txt").unwrap()
}
