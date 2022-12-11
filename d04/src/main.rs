use std::env;
use std::fs;

mod section_assignments;

use section_assignments::SectionAssignmentPair;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).unwrap();

    let mut number_of_fully_overlapping_pairs = 0;
    let mut number_of_overlapping_pairs = 0;

    for pair in input.lines() {
        let sap = SectionAssignmentPair::from_string(pair);
        if sap.full_range_is_overlapping() {
            number_of_fully_overlapping_pairs += 1;
        }
        if sap.is_overlapping() {
            number_of_overlapping_pairs += 1;
        }
    }

    println!("🎄 Number of fully overlapping assignment pairs: {number_of_fully_overlapping_pairs}");
    println!("🎄 Number of overlapping assignment pairs: {number_of_overlapping_pairs}");
}
