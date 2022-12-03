pub mod elf_carriers;

use std::env;
use std::fs;

use crate::elf_carriers::elf_carriers::ElfCarriers;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).unwrap();

    let elf_carriers = ElfCarriers::new(&input);

    println!(
        "Fattest elf carrier is {:#?}",
        elf_carriers.get_fattest_elf()
    );
}
