use std::env;
use std::fs;

pub mod nodes;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).unwrap();

    for line in input.lines() {
        println!("{}", line);
    }

    println!("🎄");
}
