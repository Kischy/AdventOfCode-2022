pub mod rps_score_calculator;

use std::env;
use std::fs;

use rps_score_calculator::rps_score_calculator::RPSCalculator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).unwrap();

    let mut rps_score_calculator = RPSCalculator::new();
    let mut total_score = 0;
    for line in input.lines() {
        total_score += rps_score_calculator.calculate_score(line);
    }

    println!("Total score {}", total_score);

}
