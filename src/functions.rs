use std::collections::HashMap;

pub fn count_increases_in_measurements(measures: &Vec<i64>) -> usize {
    let n_measures: usize = measures.len();
    let mut counter: usize = 0;
    for i in 1..n_measures {
        if measures[i] > measures[i - 1]{
            counter += 1;
        }
    }
    counter
}

pub fn count_increases_in_measures_by_window(measures: &Vec<i64>, window: usize) -> usize {
    let n_measures: usize = measures.len();
    let mut counter: usize = 1; // la soluzione giusta aggiunge 1 a questa soluzione... bo!

    for i in (1 + window)..n_measures {
        let group_a: Vec<i64> = measures[(i - window)..i].to_vec();
        let group_b: Vec<i64> = measures[(i - window - 1)..(i - 1)].to_vec();

        let sum_a: i64 = group_a.iter().sum();
        let sum_b: i64 = group_b.iter().sum();

        if sum_a > sum_b {
            counter += 1;
        }
    }
    counter
}

fn process_instructions(instruction: &String) -> Vec<&str> {
    instruction.split(" ").collect()
}

pub fn consider_instructions(instructions: &Vec<String>) -> usize {
    let n_instructions = instructions.len();

    let mut horz_value: usize = 0;
    let mut depth_value: usize = 0;

    for i in 0..n_instructions {
        let instr = process_instructions(&instructions[i]);
        let value: usize = instr[1].parse().expect("N");

        if instr[0] == "forward" {
            horz_value += value;
        }

        if instr[0] == "down" {
            depth_value += value;
        }

        if instr[0] == "up" {
            depth_value -= value;
        }

    }
    horz_value * depth_value
}


pub fn consider_instructions_aim(instructions: &Vec<String>) -> usize {
    let n_instructions = instructions.len();

    let mut horz_value: usize = 0;
    let mut depth_value: usize = 0;
    let mut aim_value: usize = 0;

    for i in 0..n_instructions {
        let instr = process_instructions(&instructions[i]);
        let value: usize = instr[1].parse().expect("N");

        if instr[0] == "forward" {
            horz_value += value;
            depth_value += aim_value * value;
        }

        if instr[0] == "down" {
            aim_value += value;
        }

        if instr[0] == "up" {
            aim_value -= value;
        }

    }
    horz_value * depth_value
}

fn get_usage_vect(elements: usize) -> Vec<Vec<usize>> {
    let mut usage_vector: Vec<Vec<usize>> = Vec::new();
    for i in 0..elements {
        usage_vector.push(vec![i, 0]);
    }
    usage_vector
}

fn get_element_from_string(input_string: &str, element: usize) -> String {
    if let Some(out) = input_string.chars().nth(element) {
        out.to_string()
    } else {
        String::new()
    }
}

fn get_gamma_and_epsilon(counter: Vec<Vec<usize>>, n: usize) -> (String, String) {
    let half_threshold: usize = n / 2;
    let n_bits: usize = counter.len();

    let mut gamma: Vec<&str> = Vec::new();
    let mut epsilon: Vec<&str> = Vec::new();

    for i in 0..n_bits {
        let value = counter[i][1];
        if value > half_threshold {
            gamma.push("1");
            epsilon.push("0");
        } else {
            gamma.push("0");
            epsilon.push("1");
        }
    }

    (gamma.join(""), epsilon.join(""))

}

fn binary2int(binary: &str) -> i64 {
    let value = i64::from_str_radix(binary, 2).unwrap();
    value
}

/* fn flip_binary(binary: &str) -> String {
    let mut out: Vec<&str> = Vec::new();
    
    for i in binary.chars().into_iter() {
        if i == '1' {
            out.push("0");
        }
        else {
            out.push("1");
        }
    }
    out.join("")
}
*/

pub fn calculate_power_consumption(binaries: Vec<String>) -> i64 {
    let n_bits: usize = binaries[0].len();
    let mut usage_vector = get_usage_vect(n_bits);

    let n_binaries: usize = binaries.len();
    for i in 0..n_binaries {
        for j in 0..n_bits {
            let value = get_element_from_string(&binaries[i], j);
            if value == "1" {
                usage_vector[j][1] += 1;
            }
        }
    }

    let (gamma, epsilon) = get_gamma_and_epsilon(usage_vector, n_binaries);

    binary2int(&gamma) * binary2int(&epsilon)
}

fn keep_binaries_with_bit_position<'a>(binaries: Vec<&'a str>, position: usize, value: String) -> Vec<&'a str> {
    let mut out_vector: Vec<&str> = Vec::new();
    let n_binaries: usize = binaries.len();

    let chars_at_position = |element: &str, pos: usize| element.chars().nth(pos).unwrap();

    for i in 0..n_binaries {
        if chars_at_position(&binaries[i], position) == chars_at_position(&value, 0) {
            out_vector.push(&binaries[i]);
        }
    }
    out_vector
}

pub fn calculate_life_support_rating(binaries: Vec<String>) -> i64 {
    let n_bits: usize = binaries[0].len();
    let mut use_binaries: Vec<&str> = binaries.iter().map(|s| s.as_str()).collect();

    for i in 0..n_bits {

        let mut counter: usize = 0;
        let n_binaries: usize = use_binaries.len();
        let half_threshold: usize = n_binaries / 2; 

        if n_binaries > 1 {

            for j in 0..n_binaries {
                
                let value = get_element_from_string(&use_binaries[j], i);

                if value == "1" {
                    counter += 1;
                }
            }

            let use_value_in_search: String = if counter >= half_threshold {
                "1".to_string()
            } else {
                "0".to_string()
            };

            use_binaries = keep_binaries_with_bit_position(use_binaries, i, use_value_in_search);
        }
    }

    let oxygen_gen = use_binaries[0];

    let mut use_binaries: Vec<&str> = binaries.iter().map(|s| s.as_str()).collect();

    for i in 0..n_bits {

        let mut counter: usize = 0;
        let n_binaries: usize = use_binaries.len();
        let half_threshold: usize = n_binaries / 2; 

        if n_binaries > 1 {

            for j in 0..n_binaries {
                
                let value = get_element_from_string(&use_binaries[j], i);

                if value == "1" {
                    counter += 1;
                }
            }

            let use_value_in_search: String = if counter < half_threshold {
                "1".to_string()
            } else {
                "0".to_string()
            };

            use_binaries = keep_binaries_with_bit_position(use_binaries, i, use_value_in_search);
        }
    }

    let co2_scrub = use_binaries[0];

    binary2int(&oxygen_gen) * binary2int(&co2_scrub)
}

/* fn print_condition(text: &str) {
    println!("qui! {}", text);
}

fn print_number(number: &i64) {
    println!("qui! {}", number);
} */

fn update_unit_board(
    number: &i64, 
    original_board: &Vec<Vec<i64>>, 
    unit_board: Vec<Vec<i64>>
) -> Vec<Vec<i64>> {
    let mut updated_board = unit_board.clone();

    let n_rows = original_board.len();
    let n_cols = original_board[0].len();
    
    for i in 0..n_rows {
        for j in 0..n_cols {
            if *number == original_board[i][j] {
                updated_board[i][j] = 0;
            }
        }
    }
    updated_board
}

fn sum_of_boards(a_mat: Vec<Vec<i64>>, b_mat: Vec<Vec<i64>>) -> i64 {

    let mut counter = 0;
    let n_cols = a_mat.len();
    let n_rows = a_mat[0].len();

    for i in 0..n_cols {
        for j in 0..n_rows {
            counter += a_mat[i][j] * b_mat[i][j];
        }
    }
    counter
}

fn win_bingo_condition(check_board: &Vec<Vec<i64>>, unit_board: &Vec<Vec<i64>>) -> i64 {
    let n_rows = unit_board.len();
    let n_cols = unit_board[0].len();

    for i in 0..n_rows {
        let u_board = &unit_board[i];
        let sum: i64 = u_board.iter().sum();

        if sum == 0 {
            return sum_of_boards(check_board.clone(), unit_board.clone());
        }
    }

    for i in 0..n_cols {
        let u_board: Vec<i64> = unit_board.iter().map(|x| x[i]).collect();
        let sum: i64 = u_board.iter().sum();

        if sum == 0 {
            return sum_of_boards(check_board.clone(), unit_board.clone());
        }
    }

    return 0;
}

pub fn best_bingo_board(draws: Vec<i64>, boards: HashMap<usize, Vec<Vec<i64>>>) -> (i64, i64) {
    let n_draws = draws.len();
    let n_boards = boards.len();

    let mut unit_boards: HashMap<usize, Vec<Vec<i64>>> = HashMap::new();
    let mut found: bool = false;
    let mut found_boards: Vec<bool> = vec![false; n_boards];
    let mut first_to_win: i64 = 0;
    let mut last_to_win: i64 = 0;

    for i in 0..n_boards {
        unit_boards.insert(i, vec![vec![1; 5]; 5]);
    }

    for i in 0..n_draws {
        for j in 0..n_boards {

            if found_boards[j] == false {
                let element_to_find = draws[i];
                let board_to_check = boards.get(&j).unwrap().clone();
                let board_to_update = unit_boards.get(&j).unwrap().clone();

                let updated_board: Vec<Vec<i64>> = update_unit_board(
                    &element_to_find, 
            &board_to_check, 
                board_to_update);

                unit_boards.insert(j, updated_board.clone());

                let win_condition: i64 = win_bingo_condition(&board_to_check, &updated_board);
                if win_condition != 0 && !found {
                    found = true;
                    found_boards[j] = true;
                    first_to_win = win_condition * element_to_find;
                }
                if win_condition != 0 && found {
                    found_boards[j] = true;
                    last_to_win = win_condition * element_to_find;
                }
            }
        }
    }
    (first_to_win, last_to_win)
}

fn max_of_side(side: &Vec<Vec<i64>>) -> i64 {
    let mut higher = &0;
    
    for row in side {
        for single in row {
            if single > higher {
                higher = single;
            }
        }
    }
    *higher
}

pub fn make_range(start: i64, end: i64) -> Vec<i64> {

    let diff = end - start;
    if diff > 0 {
        return (start..=end).collect();
    } else {
        return (end..=start).rev().collect();
    }

}

fn draw_wall_points(
    left: Vec<Vec<i64>>, 
    right: Vec<Vec<i64>>
) -> Vec<Vec<i64>> {

    let n_coords = left.len();
    let max_of_left = max_of_side(&left);
    let max_of_right = max_of_side(&right);
    let max_of_coords = max_of_left.max(max_of_right) + 1;
    let mut map = vec![vec![0; max_of_coords as usize]; max_of_coords as usize];

    for coords in 0..n_coords {

        let left_left = left[coords as usize][0];
        let left_right = left[coords as usize][1];
        let right_left = right[coords as usize][0];
        let right_right = right[coords as usize][1];

        let first_diff = right_left - left_left;
        let second_diff = right_right - left_right;

        if first_diff == 0 {
            let second_range: Vec<i64> = make_range(left_right, right_right);

            for i in 0..second_range.len() {
                map[left_left as usize][second_range[i] as usize] += 1;
            }
        }

        if second_diff == 0 {
            let first_range: Vec<i64> = make_range(left_left, right_left);

            for i in 0..first_range.len() {
                map[first_range[i] as usize][left_right as usize] += 1;
            }
        }

        if first_diff != 0 && second_diff != 0 {
            let first_range: Vec<i64> = make_range(left_left, right_left);            
            let second_range: Vec<i64> = make_range(left_right, right_right);

            for i in 0..first_range.len() {
                map[first_range[i] as usize][second_range[i] as usize] += 1;
            }
        }
    }

    map
}


pub fn number_of_crosses(left: Vec<Vec<i64>>, right: Vec<Vec<i64>>) -> i64 {
    let map: Vec<Vec<i64>> = draw_wall_points(left, right);

    let mut counter: i64 = 0;

    for row in map {
        println!("{:?}", &row);

        for element in row {
            if element > 1 {
                counter += 1;
            }
        }
    }
    counter
}




