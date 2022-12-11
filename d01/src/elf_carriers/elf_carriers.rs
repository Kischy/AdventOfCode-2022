use std::{cmp::Ordering, collections::BTreeSet};

#[derive(Clone, Debug)]
pub struct ElfCarrier {
    pub calories: u128,
    pub index: usize,
}

impl ElfCarrier {
    fn new() -> ElfCarrier {
        ElfCarrier {
            calories: 0,
            index: 0,
        }
    }
}

impl Ord for ElfCarrier {
    fn cmp(&self, other: &Self) -> Ordering {
        self.calories.cmp(&other.calories)
    }
}

impl PartialOrd for ElfCarrier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ElfCarrier {
    fn eq(&self, other: &Self) -> bool {
        (self.calories, &self.index) == (other.calories, &other.index)
    }
}

impl Eq for ElfCarrier {}

pub struct ElfCarriers {
    carriers: BTreeSet<ElfCarrier>,
}

fn calculate_carriers(elf_carriers_input: &str) -> BTreeSet<ElfCarrier> {
    let mut carriers: BTreeSet<ElfCarrier> = BTreeSet::new();
    let mut carrier = ElfCarrier::new();
    let mut index: usize = 0;
    for line in elf_carriers_input.lines() {
        if line.is_empty() {
            carriers.insert(carrier);
            carrier = ElfCarrier::new();
            index += 1;
            carrier.index = index;
        } else {
            carrier.calories += line.parse::<u128>().unwrap();
        }
    }

    carriers
}

impl ElfCarriers {
    pub fn new(elf_carriers_input: &str) -> ElfCarriers {
        ElfCarriers {
            carriers: calculate_carriers(elf_carriers_input),
        }
    }

    pub fn get_fattest_elf(&self) -> &ElfCarrier {
        self.carriers.iter().next_back().unwrap()
    }

    pub fn get_fattest_three_elfs(&self) -> Vec<ElfCarrier> {
        let mut iter = self.carriers.iter();
        vec![iter.next_back().unwrap().clone(),iter.next_back().unwrap().clone(),iter.next_back().unwrap().clone()]
    }

}

#[cfg(test)]
mod tests {
    use super::ElfCarriers;

    use indoc::indoc;

    #[test]
    fn get_fattest_elf_test() {
        let input = indoc! {"1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000"};

        let mut elf_carriers = ElfCarriers::new(input);

        assert_eq!(elf_carriers.get_fattest_elf().calories, 24000);
        assert_eq!(elf_carriers.get_fattest_elf().index, 3);
    }
}
