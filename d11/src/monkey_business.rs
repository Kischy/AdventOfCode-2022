use std::{
    cmp::Ordering,
    collections::{BTreeSet, VecDeque},
};

#[derive(Clone)]
pub struct Monkey {
    pub items: VecDeque<u64>,
    pub test_number: u64,
    pub throw_to: Vec<usize>,
    pub operation: fn(u64) -> u64,
    pub inspections: u64,
}

impl Ord for Monkey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inspections.cmp(&other.inspections)
    }
}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.inspections == other.inspections
    }
}

impl Eq for Monkey {}

pub struct MonkeyBusiness {
    pub monkeys: VecDeque<Monkey>,
    pub adjust_worry_level: Box<dyn Fn(u64) -> u64>,
}

impl MonkeyBusiness {
    pub fn new() -> MonkeyBusiness {
        MonkeyBusiness {
            monkeys: VecDeque::new(),
            adjust_worry_level: Box::new(|old| old / 3),
        }
    }

    pub fn add_monkey(&mut self, monkey: Monkey) {
        self.monkeys.push_back(monkey);
    }

    fn do_item(&mut self, monkey_id: usize, mut item: u64) {
        let mut monkey = &mut self.monkeys[monkey_id];
        item = (monkey.operation)(item);
        item = (self.adjust_worry_level)(item);
        let mut throw_to = monkey.throw_to[1];
        if item % monkey.test_number == 0 {
            throw_to = monkey.throw_to[0];
        }
        monkey.inspections += 1;

        self.monkeys[throw_to].items.push_back(item);
    }

    fn do_turn(&mut self, monkey_id: usize) {
        let items_len = self.monkeys[monkey_id].items.len();
        for _ in 0..items_len {
            let item = self.monkeys[monkey_id].items.pop_front().unwrap();
            self.do_item(monkey_id, item);
        }
    }

    pub fn do_one_round(&mut self) {
        for i in 0..self.monkeys.len() {
            self.do_turn(i);
        }
    }

    pub fn level_of_monkey_business(&self) -> u64 {
        let mut sorted_monkeys = BTreeSet::new();

        for monkey in &self.monkeys {
            sorted_monkeys.insert(monkey.clone());
        }
        let mut mon_iter = sorted_monkeys.iter();
        mon_iter.next_back().unwrap().inspections * mon_iter.next_back().unwrap().inspections
    }
}
