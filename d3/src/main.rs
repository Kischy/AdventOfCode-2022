pub mod rucksack;

use std::env;
use std::fs;
use rucksack::rucksack::get_wrongly_packed_item;
use rucksack::rucksack::get_item_priority;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).unwrap();

    let mut sum_of_priorities = 0;

    for line in input.lines() {
        let item = get_wrongly_packed_item(line);
        sum_of_priorities += get_item_priority(item);
    }

    println!("Sum of priorities of wrongly packed items {}", sum_of_priorities);

}
