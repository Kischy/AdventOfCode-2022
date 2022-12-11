pub mod rucksack;

use rucksack::rucksack::get_item_priority;
use rucksack::rucksack::get_rucksacks_badge;
use rucksack::rucksack::get_wrongly_packed_item;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).unwrap();

    let mut sum_of_priorities = 0;
    let rucksacks: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    for rucksack in rucksacks.iter() {
        let item = get_wrongly_packed_item(&rucksack);
        sum_of_priorities += get_item_priority(item);
    }

    println!(
        "Sum of priorities of wrongly packed items {}",
        sum_of_priorities
    );

    let mut sum_of_badge_priorities = 0;

    for i in (0..rucksacks.len()).step_by(3) {
        let badge = get_rucksacks_badge(&rucksacks[i], &rucksacks[i + 1], &rucksacks[i + 2]);
        sum_of_badge_priorities += get_item_priority(badge);
    }

    println!(
        "Sum of priorities of badges {}",
        sum_of_badge_priorities
    );
}
