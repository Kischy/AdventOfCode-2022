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

    let tree_fattest_elfs = elf_carriers.get_fattest_three_elfs();

    println!("Fattest three elfs are {:#?}", tree_fattest_elfs);
    println!("Calories of fattest three elfs are {}", tree_fattest_elfs.iter().map(|elf| elf.calories).sum::<u128>());
}
