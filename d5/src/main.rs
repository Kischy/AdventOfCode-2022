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

    let mut cargo_9000 = Cargo::from_string(cargo_str);
    let mut cargo_9001 = cargo_9000.clone();

    for one_move in moves.lines() {
        if one_move.is_empty() {
            continue;
        }
        cargo_9000.move_crates_9000(&one_move);
        cargo_9001.move_crates_9001(&one_move);
    }

    println!(
        "🎄 Top crates for CrateMover9000 are {}",
        cargo_9000.get_top_crates()
    );
    println!(
        "🎄 Top crates for CrateMover9001 are {}",
        cargo_9001.get_top_crates()
    );
}
