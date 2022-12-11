use std::collections::{HashMap, VecDeque};

use monkey_business::{Monkey, MonkeyBusiness};

pub mod monkey_business;

fn main() {
    let mut monkey_business = MonkeyBusiness::new();

    let m0 = Monkey {
        items: VecDeque::from(vec![74, 64, 74, 63, 53]),
        test_number: 5,
        throw_to: vec![1, 6],
        operation: |old| old * 7,
        inspections: 0,
    };
    monkey_business.add_monkey(m0);

    let m1 = Monkey {
        items: VecDeque::from(vec![69, 99, 95, 62]),
        test_number: 17,
        throw_to: vec![2, 5],
        operation: |old| old * old,
        inspections: 0,
    };
    monkey_business.add_monkey(m1);

    let m2 = Monkey {
        items: VecDeque::from(vec![59, 81]),
        test_number: 7,
        throw_to: vec![4, 3],
        operation: |old| old + 8,
        inspections: 0,
    };
    monkey_business.add_monkey(m2);

    let m3 = Monkey {
        items: VecDeque::from(vec![50, 67, 63, 57, 63, 83, 97]),
        test_number: 13,
        throw_to: vec![0, 7],
        operation: |old| old + 4,
        inspections: 0,
    };
    monkey_business.add_monkey(m3);

    let m4 = Monkey {
        items: VecDeque::from(vec![61, 94, 85, 52, 81, 90, 94, 70]),
        test_number: 19,
        throw_to: vec![7, 3],
        operation: |old| old + 3,
        inspections: 0,
    };
    monkey_business.add_monkey(m4);

    let m5 = Monkey {
        items: VecDeque::from(vec![69]),
        test_number: 3,
        throw_to: vec![4, 2],
        operation: |old| old + 5,
        inspections: 0,
    };
    monkey_business.add_monkey(m5);

    let m6 = Monkey {
        items: VecDeque::from(vec![54, 55, 58]),
        test_number: 11,
        throw_to: vec![1, 5],
        operation: |old| old + 7,
        inspections: 0,
    };
    monkey_business.add_monkey(m6);

    let m7 = Monkey {
        items: VecDeque::from(vec![79, 51, 83, 88, 93, 76]),
        test_number: 2,
        throw_to: vec![0, 6],
        operation: |old| old * 3,
        inspections: 0,
    };
    monkey_business.add_monkey(m7);
}
