use std::env;
use std::fs;

use crate::rope_movement::RopeGrid;

pub mod rope_movement;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).unwrap();

    let mut grid = RopeGrid::new();

    for line in input.lines() {
        grid.make_move(line);
    }

    println!(
        "🎄 Number of grid points visited by tail at least once: {}",
        grid.number_of_visited_points_tail()
    );
}
