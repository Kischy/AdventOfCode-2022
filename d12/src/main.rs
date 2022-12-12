use std::env;
use std::fs;

use crate::nodes::Nodes;

pub mod nodes;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).unwrap();

    let mut nodes = Nodes::from_AoC_string(&input);

    println!("🎄 {},{}", nodes.start, nodes.end);
    println!("🎄 {}", nodes.get_shortest_path_length());

    // for node in &nodes.nodes {
    //     if node.connections.len() != 4 {
    //         println!("{},{},{}", node.index, node.orig_char, node.connections.len());
    //     }
    // }

    // for node in &nodes.nodes {
    //     if node.distance_to_start == i64::MAX {
    //         print!("M,");
    //     } else {
    //         print!("{},", node.distance_to_start);
    //     }
    //     if node.index % 159 == 0 {
    //         print!("\n");
    //     }
    // }
}
