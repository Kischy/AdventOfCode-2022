use std::env;
use std::fs;

use order_checker::is_in_correct_order;

pub mod order_checker;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();
    let mut sum_of_indices = 0;
    let mut current_index = 1;

    for i in (0..input.len()).step_by(3) {
        if is_in_correct_order(input[i].as_str(), input[i + 1].as_str()) {
            sum_of_indices += current_index;
        }

        current_index += 1;
    }

    println!("🎄 {}", sum_of_indices);
}
