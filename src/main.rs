mod functions;
use functions::*;

mod utils_func;
use utils_func::*;

use std::collections::HashMap;

fn main() {
    let path: String = String::from("../questions/question_01.txt");
    let measures: Vec<i64> = read_data(&path);

    let question_01: usize = count_increases_in_measurements(&measures);
    println!("{}: {:?}", "Question #1 ", question_01);

    let question_01_b: usize = count_increases_in_measures_by_window(&measures, 3);
    println!("{}: {:?}", "Question #1B", question_01_b);

//

    let path: String = String::from("../questions/question_02.txt");
    let instructions: Vec<String> = read_instructions(&path);

    let question_02: usize = consider_instructions(&instructions);
    println!("{}: {:?}", "Question #2 ", question_02);

    let question_02_b: usize = consider_instructions_aim(&instructions);
    println!("{}: {:?}", "Question #2B", question_02_b);

//

    let path: String = String::from("../questions/question_03.txt");
    let binaries: Vec<String> = read_instructions(&path);

    let question_03: i64 = calculate_power_consumption(binaries.clone());
    println!("{}: {:?}", "Question #3 ", question_03);

    let question_03_b: i64 = calculate_life_support_rating(binaries.clone());
    println!("{}: {:?}", "Question #3B", question_03_b);

//

    let path_a: String = String::from("../questions/question_04_a.txt");
    let draws: Vec<i64> = read_draws(&path_a);

    let path_b: String = String::from("../questions/question_04_b.txt");
    let boards: HashMap<usize, Vec<Vec<i64>>> = read_boards(&path_b);

    let (question_04, question_04B): (i64, i64) = best_bingo_board(draws, boards);
    println!("{}: {:?}", "Question #4 ", question_04); 
    println!("{}: {:?}", "Question #4B", question_04B); 

// 

    let path: String = String::from("../questions/question_05.txt");
    let (left, right): (Vec<Vec<i64>>, Vec<Vec<i64>>) = read_coordinates(&path);

    let question_05: i64 = number_of_crosses(left, right, true);
    println!("{}: {:?}", "Question #5 ", question_05); 

    // 

    let path: String = String::from("../questions/question_06.txt");
    let start_state: Vec<i64> = read_draws(&path);

    let question_06: i64 = fish_simulator(&start_state, 80);
    println!("{}: {:?}", "Question #6 ", question_06); 

    let question_06B: i64 = fish_simulator_improved(&start_state, 256);
    println!("{}: {:?}", "Question #6B", question_06B); 

    // 

    let path: String = String::from("../questions/question_07.txt");
    let start_state: Vec<i64> = read_draws(&path);

    let question_07: i64 = fuel_consumption_best_position(&start_state);
    println!("{}: {:?}", "Question #7 ", question_07); 

    let question_07B: i64 = fuel_consumption_best_position_add_step(&start_state);
    println!("{}: {:?}", "Question #7B", question_07B); 

    // 

    let path: String = String::from("../questions/q8test.txt");
    match read_signals(&path) {
        Ok((input, output)) => {

            let question_08: i64 = count_simple_digits(&input, &output);
            println!("{}: {:?}", "Question #8 ", question_08); 

            let question_08B: i64 = count_hard_digits(&input, &output);
            println!("{}: {:?}", "Question #8B", question_08B); 

        }
        Err(err) => eprintln!("Error: {}", err),
    }




 }

