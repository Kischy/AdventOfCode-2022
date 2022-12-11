use std::collections::{HashMap, VecDeque};

pub struct Monkey {
    pub items: VecDeque<u64>,
    pub test_number: u64,
    pub throw_to: Vec<u64>,
    pub operation: fn(u64) -> u64,
    pub inspections: u64,
}

pub struct MonkeyBusiness {
    monkeys: VecDeque<Monkey>,
}

impl MonkeyBusiness {
    pub fn new() -> MonkeyBusiness {
        MonkeyBusiness {
            monkeys: VecDeque::new(),
        }
    }

    pub fn add_monkey(&mut self, monkey: Monkey) {
        self.monkeys.push_back(monkey);
    }
}
