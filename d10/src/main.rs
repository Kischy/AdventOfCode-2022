use std::env;
use std::fs;

use signal_strength::SignalStrengthCalculator;

pub mod signal_strength;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).unwrap();

    let mut sig_calc = SignalStrengthCalculator::new();

    for line in input.lines() {
        sig_calc.calc_instruction(line);
    }

    let interesting_cycles = vec![20,60,100,140,180,220];

    let mut sig_sum = 0;

    for cy in interesting_cycles {
        sig_sum += *sig_calc.signal_strengths.entry(cy).or_insert(0);
    }

    println!(
        "🎄 Sum of signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles {}",
        sig_sum
    );
}
