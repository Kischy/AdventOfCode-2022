pub mod rps_score_calculator;

use std::env;
use std::fs;

use rps_score_calculator::rps_score_calculator::RPSCalculator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).unwrap();

    let mut rps_score_calculator = RPSCalculator::new();
    let mut total_score_part1 = 0;
    let mut total_score_part2 = 0;
    for line in input.lines() {
        total_score_part1 += rps_score_calculator.calculate_score_part1(line);
        total_score_part2 += rps_score_calculator.calculate_score_part2(line);
    }

    println!("Total score part 1 {}", total_score_part1);
    println!("Total score part 2 {}", total_score_part2);
}
