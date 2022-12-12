use std::env;
use std::fs;

use crate::nodes::Nodes;

pub mod nodes;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).unwrap();

    let mut nodes = Nodes::from_AoC_string(&input);

    println!(
        "🎄 Shortest path part 1: {}",
        nodes.get_shortest_path_length().unwrap()
    );
    println!(
        "🎄 Shortest path from letter a: {}",
        nodes.get_shortest_path_from_letter_a()
    );
}
