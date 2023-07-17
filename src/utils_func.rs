use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn read_data(path: &String) -> Vec<i64> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut numbers = Vec::new();

    for line in reader.lines() {
        if let Ok(num) = line {
            if let Ok(parsed_num) = num.parse::<i64>() {
                numbers.push(parsed_num);
            }
        }
    }

    numbers
}

pub fn read_instructions(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut instructions = Vec::new();

    for line in reader.lines() {
        if let Ok(instruction) = line {
            instructions.push(instruction);
        }
    }

    instructions
}

pub fn read_draws(path: &str) -> Vec<i64> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut draws = Vec::new();

    for line in reader.lines() {
        if let Ok(single_draws) = line {
            for x in single_draws.split(",") {
                if let Ok(number) = x.trim().parse::<i64>() {
                    draws.push(number);
                }
            }
        }
    }
    draws
}

fn rearrange_boards(boards: HashMap<usize, Vec<i64>>) -> HashMap<usize, Vec<Vec<i64>>> {
    let n_boards: usize = boards.len();

    let mut arranged_boards: HashMap<usize, Vec<Vec<i64>>> = HashMap::new();

    let fixed_length: i32 = 5;

    for i in 0..n_boards {
        let board = boards.get(&i).unwrap();
        let n_elements = board.len();

        let mut upper: Vec<Vec<i64>> = vec![vec![0; 5]; 5];

        for j in 0..n_elements {
            let idx = j as i64 / fixed_length as i64;
            let idy = j as i64 % fixed_length as i64;

            upper[idx as usize][idy as usize] = board[j];
        }
        arranged_boards.insert(i, upper);
    }
    arranged_boards
}

pub fn read_boards(path: &str) -> HashMap<usize, Vec<Vec<i64>>> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut boards: HashMap<usize, Vec<i64>> = HashMap::new();
    let mut upper: Vec<i64> = Vec::new();

    let mut counter = 0;

    for line in reader.lines() {
        if let Ok(line_of_board) = line {
            let trimmed_line = line_of_board.trim();
            if trimmed_line.is_empty() {
                boards.insert(counter, upper);
                counter += 1;
                upper = Vec::new();
            } else {
                for element in trimmed_line.split_whitespace() {
                    if let Ok(x) = element.trim().parse::<i64>() {
                        upper.push(x);
                    }
                }
            }
        }
    }
    rearrange_boards(boards)
}

pub fn read_coordinates(path: &str) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut left: Vec<Vec<i64>> = Vec::new();
    let mut right: Vec<Vec<i64>> = Vec::new();

    for line in reader.lines() {
        if let Ok(single_draws) = line {
            let pairs: Vec<&str> = single_draws.split(" -> ").collect();

            if pairs.len() != 2 {
                continue;
            }

            let left_elements: Vec<i64> = pairs[0]
                .split(',')
                .filter_map(|x| x.trim().parse().ok())
                .collect();
            let right_elements: Vec<i64> = pairs[1]
                .split(',')
                .filter_map(|x| x.trim().parse().ok())
                .collect();

            left.push(left_elements);
            right.push(right_elements);
        }
    }

    (left, right)
}
    


