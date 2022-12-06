use std::env;
use std::fs;

pub mod marker_finder;
use marker_finder::find_marker;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).unwrap();

    println!("🎄 First marker at {}", find_marker(&input));
}
