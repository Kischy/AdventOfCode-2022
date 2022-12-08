use std::env;
use std::fs;

use crate::trees::Trees;

pub mod trees;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).unwrap();

    let trees = Trees::from_string(&input);

    println!("🎄 Number of visible trees: {}", trees.number_of_visible());
    println!("🎄 Max scenic view: {}", trees.highest_scenic_view());
}
