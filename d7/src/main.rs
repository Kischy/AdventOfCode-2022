use std::env;
use std::fs;

use crate::filesystem::FileSystem;

pub mod directory;
pub mod file;
pub mod filesystem;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).unwrap();

    let mut fs = FileSystem::from_string(input.as_str());

    println!(
        "🎄 Sum of size of dirs with at most size 100000: {}",
        fs.get_sum_of_sizes(100000)
    );
    println!(
        "🎄 Size of smallest dir to delete: {}",
        fs.get_size_of_smallest_deletable_dir(70000000, 30000000)
    );
}
