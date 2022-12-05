use itertools::Itertools;

pub fn day5_part_1() -> String {
    let data = get_input();
    let lines: Vec<&str> = data.lines().collect();
    let mut instruction_line_start: usize = 0;
    let mut crates: Vec<Vec<char>> = Vec::new();
    let amt = lines[0].len()/4;

    for _c in 0..amt+1 {
        crates.push(Vec::new());
    }

    get_initial_crates(&mut crates, &lines,&mut instruction_line_start);

    parse_instructions(&mut crates, &lines, &mut instruction_line_start, false);

    let mut ans = String::from("");
    for container in crates {
        ans.push(*container.last().unwrap());
    }

    ans
}

pub fn day5_part_2 () -> String {
    let data = get_input();
    let lines: Vec<&str> = data.lines().collect();
    let mut instruction_line_start: usize = 0;
    let mut crates: Vec<Vec<char>> = Vec::new();
    let amt = lines[0].len()/4;

    for _c in 0..amt+1 {
        crates.push(Vec::new());
    }

    get_initial_crates(&mut crates, &lines,&mut instruction_line_start);

    parse_instructions(&mut crates, &lines, &mut instruction_line_start, true);

    let mut ans = String::from("");
    for container in crates {
        ans.push(*container.last().unwrap());
    }

    ans 
}

fn parse_instructions(containers: &mut [Vec<char>], lines: &[&str], instr_line: &mut usize, day_2: bool) {
    for (index, line) in lines.iter().enumerate() {
        if index < *instr_line || line == &"" {
            continue;
        }

        let instr: Vec<&str> = line.split(char::is_whitespace).collect();
        let move_amount: i32 = instr[1].parse().unwrap();
        let from: i32 = instr[3].parse().unwrap();
        let to: i32 = instr[5].parse().unwrap();

        if !day_2 {
            for _b in 0..move_amount {

                let temp = &containers[(from-1) as usize].pop().unwrap();
                let _ = &containers[(to-1) as usize].push(*temp);
            }
        } else {
            let mut temp_vec: Vec<char> = Vec::new();

        for _b in 0..move_amount {

            let temp = &containers[(from-1) as usize].pop().unwrap();
            //temp = &containers[(to-1) as usize].push(*temp);
            temp_vec.push(*temp);
        }
        temp_vec.reverse();

        for (_i, t) in temp_vec.into_iter().enumerate() {
            let _ = containers[(to-1) as usize].push(t);
        }
        }

    }
}

fn get_initial_crates (containers: &mut Vec<Vec<char>>, lines: &[&str], instr_line: &mut usize) {
    for (line_index, line) in lines.iter().enumerate() {
        // stop if line contains numbers
        if line.is_empty() {
            *instr_line = line_index;
            break;
        }

        let chunks = to_chunks(line, 4);

        for (i, chunk) in chunks.iter().enumerate() {
            for char in chunk.chars() {
                if char::is_alphabetic(char) {
                    containers[i].push(char);
                }
            }
        }

    }

    for container in containers {
        container.reverse();
    }
}

pub fn to_chunks(string: &str, chunk_size: usize) -> Vec<String> {
    let mut sections = Vec::new();
    
    for chunk in &string.chars().chunks(chunk_size) {
        sections.push(String::from_iter(chunk))
    }
    
    sections
}

fn get_input () -> String {
    std::fs::read_to_string("src/days/input/5.txt").unwrap()
}