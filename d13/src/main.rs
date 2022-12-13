use std::env;
use std::fs;
use std::iter::zip;

use order::is_in_correct_order;
use order::value_compare;

pub mod order;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();
    let mut sum_of_indices = 0;
    let mut current_index = 1;

    let divider1 = "[[2]]";
    let divider2 = "[[6]]";

    let mut packets = vec![divider1, divider2];

    for i in (0..input.len()).step_by(3) {
        if is_in_correct_order(input[i].as_str(), input[i + 1].as_str()) {
            sum_of_indices += current_index;
        }

        packets.push(input[i].as_str());
        packets.push(input[i + 1].as_str());

        current_index += 1;
    }

    println!(
        "🎄 Sum or indices of unordered packages: {}",
        sum_of_indices
    );

    packets.sort_by(|&l, &r| {
        let ljson: serde_json::Value = serde_json::from_str(l).unwrap();
        let rjson: serde_json::Value = serde_json::from_str(r).unwrap();
        if let Some(ord) = value_compare(&ljson, &rjson) {
            ord
        } else {
            panic!("Value not parsable");
        }
    });

    println!(
        "🎄 Product of devider packages: {}",
        (packets.iter().position(|&s| s == divider1).unwrap() + 1)
            * (packets.iter().position(|&s| s == divider2).unwrap() + 1)
    );
}
