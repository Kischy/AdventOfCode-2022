use std::env;
use std::fs;

use crate::rope_movement::RopeGrid;

pub mod rope_movement;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).unwrap();

    let mut short_rop_grid = RopeGrid::new(2);
    let mut longer_rope_grid = RopeGrid::new(10);

    for line in input.lines() {
        short_rop_grid.make_move(line);
        longer_rope_grid.make_move(line);
    }

    println!(
        "🎄 Number of grid points visited by short ropes tail at least once: {}",
        short_rop_grid.number_of_visited_points_tail()
    );

    println!(
        "🎄 Number of grid points visited by long ropes tail at least once: {}",
        longer_rope_grid.number_of_visited_points_tail()
    );
}
