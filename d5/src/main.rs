use std::env;
use std::fs;

pub mod cargo;
use cargo::Cargo;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let inputs: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    let cargo_str = &inputs[0];
    let moves = &inputs[1];

    let mut cargo = Cargo::from_string(cargo_str);

    for one_move in moves.lines() {
        if one_move.is_empty() {
            continue;
        }
        cargo.move_crates(&one_move);
    }

    println!("🎄 All crates {:?}", cargo.get_crates());
    println!("🎄 Top crates are {}", cargo.get_top_crates());
}
